# Bitcoin Wallet

This guide helps you set up a Bitcoin node locally and run a Rust project that connects to the node.

## Prerequisites

- Rust installed on your system. You can install Rust from [rustup.rs](https://rustup.rs/).
- Bitcoin Core installed and synced to the network. Download Bitcoin Core from [bitcoin.org](https://bitcoin.org/en/download) and follow the installation instructions.

## Setting Up Bitcoin Node

1. [Build](https://github.com/bitcoin/bitcoin/blob/master/doc/build-unix.md) Bitcoin Core from source according to your operating system.
2. Run Bitcoin Core and let it synchronize with the network. This process may take some time depending on your internet connection and hardware.
3. Once Bitcoin Core is fully synchronized, you'll have a fully operational Bitcoin node.

## Running the Rust Project

1. Clone this repository to your local machine:

    ```bash
    git clone git@github.com:isaack-njama/bitcoin-wallet.git
    ```

2. Navigate to the project directory:

    ```bash
    cd bitcoin-wallet
    ```

3. Install dependencies and build the Rust project:

    ```bash
    cargo build
    ```

4. Run Bitcoin Core (On a separate terminal):

    ```bash
    bitcoind
    ```

5. Run the Rust project:

    ```bash
    cargo run
    ```

    This will execute the Rust project, which should establish a connection to your local Bitcoin node.

## Configuration

- The Rust project assumes that the Bitcoin node is running locally on testnet and the RPC port (`http://localhost:18332`). If your node is running on a different port or host, you'll need to adjust the configuration in the Rust project accordingly.

## Contributing

Contributions are welcome! If you find any issues or want to improve this guide, feel free to open a pull request.

## License

This project is licensed under the [MIT License](./LICENSE).
