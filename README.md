# osu-markov

`osu-markov` is a Twitter bot for the osu! community that fetches data from the IRC channel `#osu`, generates new messages using a Markov chain, and sends them to Twitter.

## Features

- Connects to the IRC server `irc.ppy.sh` to read messages from the `#osu` channel.
- Stores messages in an SQLite database.
- Trains a Markov chain model with the collected messages to generate new, coherent messages.
- Posts generated messages to Twitter at random intervals.

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
    TWITTER_API=
    ```

    The names are self-explanatory.

### Usage

To start the bot, run:

```sh
cargo run
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
