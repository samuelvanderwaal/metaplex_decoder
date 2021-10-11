# Simple Metaplex Decoder (WIP)

## Install From Source

Install [Rust](https://www.rust-lang.org/tools/install).

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Clone the source:

```bash
git clone git@github.com:samuelvanderwaal/metaplex_decoder.git
```

Change directory and install or build with Rust:

```bash
cd metaplex_decoder
```

```bash
cargo install --path ./
```

or

```bash
cargo build --release
```

## Example Usage

Create a settings file in a `configuration` directory in the same directory you run the code.

// settings.json
```json
{
  "network": "https://api.mainnet-beta.solana.com",
  "mint_accounts": ["6RB1jyer1XKJJ6X3RdmoXLV1ixdojXLjsSCsFBPLWsam"]
}
```

Run the program:

```
./metaplex_decoder
```

This will loop over all the specified mint accounts and create a metadata file with all the fields from the Rust `Data` struct. 