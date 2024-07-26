# osu-markov

`osu-markov` is a Twitter bot for the osu! community that fetches data from the IRC channel `#osu`, generates new messages using a Markov chain, and sends them to Twitter.

## Getting Started

### Installation

1. **Clone the repository:**

    ```sh
    git clone https://github.com/yorunoken/osu-markov.git
    cd osu-markov
    ```

2. **Install dependencies:**

    ```sh
    cargo build
    ```


### Environment variables

1. **Update env file:**

    Edit the `.env.example` and delete the `.example` off of it.

    ```
    USERNAME=
    SERVER=
    PASSWORD=
    PORT=
    TWITTER_CONSUMER_KEY=
    TWITTER_CONSUMER_SECRET=
    TWITTER_ACCESS_TOKEN=
    TWITTER_ACCESS_SECRET=
    ```

    The names are self-explanatory.

### Usage

To start the bot, run:

```sh
cargo run
```

### Compiling for aarch64

1. **Download and install cross**
    navigate to [cross's github repository](https://github.com/cross-rs/cross) and follow instructions

2. **Use the aarch64-unknown-linux-gnu target**
    ```sh
    cross build -r --target aarch64-unknown-linux-gnu
    ```

The bot will:

1. Create a database for message storage
2. Connect to the IRC server and join the specified channel.
3. Read messages from the IRC channel.
4. Store the messages in the database.
5. Periodically generate new messages using the Markov chain model.
6. Post the generated messages to Twitter.

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

For any questions or support, please open an issue on this repository.
