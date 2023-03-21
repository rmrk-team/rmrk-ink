# RMRK ink!
[![workflow][a1]][a2] [![stack-exchange][s1]][s2] [![discord][d1]][d2] [![built-with-ink][i1]][i2] [![License][ap1]][ap2]

[s1]: https://img.shields.io/badge/click-white.svg?logo=StackExchange&label=ink!%20Support%20on%20StackExchange&labelColor=white&color=blue
[s2]: https://substrate.stackexchange.com/questions/tagged/ink?tab=Votes
[a1]: https://github.com/swanky-dapps/nft/actions/workflows/test.yml/badge.svg
[a2]: https://github.com/rmrk-team/rmrk-ink/actions/workflows/test.yml
[d1]: https://img.shields.io/discord/722223075629727774?style=flat-square&label=discord
[d2]: https://discord.gg/Z3nC9U4
[i1]: https://github.com/swanky-dapps/nft/blob/main/.images/ink.svg
[i2]: https://github.com/paritytech/ink
[ap1]: https://img.shields.io/badge/License-Apache%202.0-blue.svg
[ap2]: https://opensource.org/licenses/Apache-2.0

Implementation of RMRK protocol in ink! Smart contract language


## Quick start

1. Make sure you have the [latest cargo contract](https://crates.io/crates/cargo-contract)


2. Clone the repository

```sh
git clone https://github.com/rmrk-team/rmrk-ink.git
```

3. Compile & Build

```sh
cd ./rmrk-ink/contracts/rmrk
cargo +nightly contract build --release
```

3. Run ink! unit tests

```sh
cargo test
```

4. Integration test
Start local test node. Recommended [swanky-node](https://github.com/AstarNetwork/swanky-node) version 1.2 or higher. Download binary or run compiled version
```sh
./swanky-node --dev --tmp
```
To run tests:

```sh
yarn
yarn compile
yarn test
````

