# Rust Blockchain

```txt
blockchain 0.1.0

USAGE:
    blockchain [OPTIONS] --port <port>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -p, --peer <peers>...
    -P, --port <port>
```

## Howto

To start the first client listening on port 1234 run:

```sh
cargo run -- --port 1234 &
```

Then start the second client and connect it to the first one:

```sh
cargo run -- --port 1235 --peer localhost:1234
```

It will generate 2 blocks and send them to the first client.
Then the first client will validate the data and print a message to the console.
