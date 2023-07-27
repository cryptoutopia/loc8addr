
# About
Loc8addr takes an xpub and a position and return BTC address at given position. The program only supports segwit addresses and if position is not given it returns the first address. If `--testnet` is passed it will generate testnet addresses.
## Use case
Simple tool to prevent address reuse in automation (eg. new address for webshop orders ).

## Usage
```
Usage: loc8addr [OPTIONS] --xpub <XPUB>

Options:

--xpub <XPUB>
--position <POSITION> [default: 0]
--testnet
-h, --help Print help
```
## Useful links

* [descriptor templates](https://docs.rs/bdk/latest/bdk/descriptor/template/index.html)

* [slip132](https://docs.rs/slip132/0.10.0/slip132/trait.FromSlip132.html)
