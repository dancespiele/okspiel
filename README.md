# Okspiel (1º Phase)

Desktop application to handle remotely multiple [Okcash full node wallets](https://github.com/okcashpro/okcash/releases)

## How Okspiel works

For now only is tested in Linux but should work in Mac. In Windows could be not however working for compatibility is part of the Road Map

### Requirements

* Rustup (you can install from here https://www.rust-lang.org/learn/get-started)
* For now you need to compile [Okcash full node wallet](https://github.com/okcashpro/okcash) from master repository until the version 6.9.0.6 is released  

### Installation

From terminal:

1. `git clone https://github.com/dancespiele/okspiel.git`
2. `cd okspiel`
3. `cargo build --release`
4. Copy the binary from `okspiel/target/release/okspiel` to `~/.cargo/bin`
5. Enjoy!

**Note**: I will publish the fist crate version `0.1.0` when the version `0.3.0` of `Iced` is released. You can see an open issue [here](https://github.com/hecrj/iced/issues/706#issuecomment-779251854)

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

## Run test

You need to include `.env` in the root folder of your project with the next variables:

```
URL=[Your domain]
ACCOUNT=[Your account name of your wallet]
RPCUSER=[the user of the rpc of your wallet]
RPCPASSWORD=[the password of the rpc of your wallet]
PHRASE=[the phrase of your wallet]
ADDRESS_TO_SEND_TEST_AMOUNT=[address for receiving test transaction]
```

After `.env` file is included, go to okspiel sources with the terminal and execute `cargo test`

**Warning**: there is a test for send transaction that will send a quantity of 0,01 $OK + fees to the address that you set in `ADDRESS_TO_SEND_TEST_AMOUNT`

### Where Okspiel is saving the data connection

Once that Okspiel is running for the first time the app create a hide folder in your home directory which calls `.okspiel`

## Support

Please if you find some bug or you have suggestions open an issue or ping me in [Okcash discord general channel](https://discord.gg/qdgzEfck)

## Road map

- [x] Connection to multiple wallets
- [x] Wallet info, Receive and Send amount
- [ ] Compatibility to another OS (only is tested in Linux)
- [ ] Create executables for different Linux distributions, Mac and Windows
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

If you want to support my contribution with Okcash ecosystem you can send tips to @spielcrypto in [Discord](https://discord.gg/EEfySvrs)
or you can send Okcash to the address `PMRhm1Zkt8fgBWjK6GKviXuTTr5ftEdQtx`

Thanks for your support!

## License

Okspiel is MIT licensed. See [license](LICENSE.md) 