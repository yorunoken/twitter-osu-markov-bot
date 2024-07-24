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

    Edit the `Secrets_example.toml` and delete the `_example` off of it.

    ```
    USERNAME =
    SERVER =
    PASSWORD =
    PORT =
    TWITTER_CONSUMER_KEY =
    TWITTER_CONSUMER_SECRET =
    TWITTER_ACCESS_TOKEN =
    TWITTER_ACCESS_SECRET =
    ```

    The names are self-explanatory.

### Usage

To start the bot, you a [shuttle](https://shuttle.rs) account, as I'm using it to host this bot.
You can also do without it, just install [cargo-shuttle](https://archlinux.org/packages/extra/x86_64/cargo-shuttle/) from the official Arch repo, and run:

```sh
cargo-shuttle run
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
