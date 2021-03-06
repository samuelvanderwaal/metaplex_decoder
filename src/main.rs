use anyhow::Result;
use serde::Serialize;
use serde_json::json;
use solana_client::rpc_client::RpcClient;
use solana_program::borsh::try_from_slice_unchecked;
use solana_sdk::pubkey::Pubkey;
use spl_token_metadata::state::Metadata;
use std::fs::File;
use std::str::FromStr;

use metaplex_decoder::configuration::setup_config;
// use metaplex_decoder::metadata::Metadata;

const METAPLEX_PROGRAM_ID: &'static str = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s";

#[derive(Debug, Serialize)]
pub struct JSONCreator {
    pub address: String,
    pub verified: bool,
    pub share: u8,
}

fn main() -> Result<()> {
    let settings = setup_config()?;

    let connection = RpcClient::new(settings.network);

    for mint_account in settings.mint_accounts {
        let mint_pubkey = Pubkey::from_str(&mint_account)?;
        let metadata_pda = match get_metadata_pda(mint_pubkey) {
            Some(pubkey) => pubkey,
            None => panic!("No metaplex account found"),
        };
        println!("Metadata Account: {}", metadata_pda);

        let account_data = connection
            .get_account_data(&metadata_pda)
            .expect("Failed to get account data.");

        let metadata: Metadata = try_from_slice_unchecked(&account_data)?;

        println!("Metadata creators: {:?}\n", metadata.data.creators);

        let mut creators: Vec<JSONCreator> = Vec::new();

        if let Some(c) = metadata.data.creators {
            creators = c
                .iter()
                .map(|c| JSONCreator {
                    address: c.address.to_string(),
                    verified: c.verified,
                    share: c.share,
                })
                .collect::<Vec<JSONCreator>>();
        }

        let nft_metadata = json!({
            "name": metadata.data.name.to_string().trim_matches(char::from(0)),
            "symbol": metadata.data.symbol.to_string().trim_matches(char::from(0)),
            "seller_fee_basis_points": metadata.data.seller_fee_basis_points,
            "uri": metadata.data.uri.to_string().trim_matches(char::from(0)),
            "creators": [creators],
        });
        let mut file = File::create(format!("metadata/{}.json", mint_account))?;
        serde_json::to_writer(&mut file, &nft_metadata)?;
    }

    Ok(())
}

fn get_metadata_pda(mint_account: Pubkey) -> Option<Pubkey> {
    let metaplex_pubkey = METAPLEX_PROGRAM_ID
        .parse::<Pubkey>()
        .expect("Failed to parse Metaplex Program Id");

    let seeds = &[
        "metadata".as_bytes(),
        metaplex_pubkey.as_ref(),
        mint_account.as_ref(),
    ];

    let (pda, _) = Pubkey::find_program_address(seeds, &metaplex_pubkey);
    Some(pda)
}
