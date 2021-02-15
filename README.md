# Okspiel (1º Phase)

Desktop application to handle remotely multiple [Okcash full node wallets](https://github.com/okcashpro/okcash/releases)

## How Okspiel works

For now only is tested in Linux but should work in Mac. In Windows could be not however working for compatibility is part of the Road Map

### Requirements

* Rustup (you can install from here https://www.rust-lang.org/learn/get-started)
* For now you need to compile [Okcash full node wallet](https://github.com/okcashpro/okcash) from master repository until the version 6.9.0.6 is released  

### Installation

`cargo install okspiel`

### Start the app

execute `okspiel` in you terminal

### Connecting with remote wallet full node

1. Open the okcash config file of your wallet full node where you want to connect
2. Include this:

```
listen=1
server=1
daemon=1
whitelist=[IP_WHERE_IS_THE_OKSPIEL]
rpcallowip=[IP_WHERE_IS_THE_OKSPIEL]
```

## Support

Please if you find some bug or you have suggestions open an issue or ping me in [Okcash discord general channel](https://discord.gg/qdgzEfck)

## Road map

- [x] Connection to multiple wallets
- [x] Wallet info, Receive and Send amount
- [ ] Improve UX
- [ ] List of transactions
- [ ] handler multiple accounts instead of only be possible to connect to one account
- [ ] Add and update phrase
- [ ] Create accounts
- [ ] Move amounts between accounts
- [ ] Backups
- [ ] Import wallets
- [ ] Okexplorer

## I appreciate your support

If you want to support my contribution with Okcash ecosystem you can send tip to @spielcrypto in [Discord](https://discord.gg/EEfySvrs)
or you can send Okcash to the address `PMRhm1Zkt8fgBWjK6GKviXuTTr5ftEdQtx`

## License

Okspiel is MIT licensed. See [license](LICENSE.md) 