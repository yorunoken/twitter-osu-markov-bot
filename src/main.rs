use dotenv::dotenv;
use std::env;

use rusqlite::Connection;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::time::Duration;

mod markov_chain;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the environment variables
    dotenv().ok();

    let conn = Connection::open("data.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS messages (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        content TEXT NOT NULL,
        nickname TEXT NOT NULL,
        channel TEXT NOT NULL
        );",
        [],
    )?;
    // Insert dummy data
    conn.execute(
        "INSERT INTO messages (content, nickname, channel)
    SELECT 'dummy content', 'dummy nickname', '#osu'
    WHERE NOT EXISTS (SELECT 1 FROM messages);",
        [],
    )?;

    // Define config
    let username = env::var("USERNAME").expect("`USERNAME` is not defined in .env");
    let server = env::var("SERVER").expect("`SERVER` is not defined in .env");
    let password = env::var("PASSWORD").expect("`PASSWORD` is not defined in .env");
    let port = env::var("PORT").expect("`PORT` is not defined in .env");
    let channel = "#osu";

    // Create connection
    let address = format!("{}:{}", server, port);
    let stream = TcpStream::connect(address).await?;
    let (read_half, mut write_half) = stream.into_split();

    // Pass in password, nick, and user
    write_half
        .write_all(format!("PASS {}\r\n", password).as_bytes())
        .await?;
    write_half
        .write_all(format!("NICK {}\r\n", username).as_bytes())
        .await?;
    write_half
        .write_all(format!("USER {} 0 * :{}\r\n", username, username).as_bytes())
        .await?;

    // Join channel
    write_half
        .write_all(format!("JOIN {}\r\n", channel).as_bytes())
        .await?;

    // Get content
    let mut reader = BufReader::new(read_half).lines();

    tokio::select! {
        _ = async {
            while let Some(line) = reader.next_line().await? {
                if line.contains("PING") {
                    // Respond to PING to keep the connection alive
                    let response = line.replace("PING", "PONG");
                    write_half.write_all(response.as_bytes()).await?;
                } else {
                    if let Some(message) = utils::parse_irc_msg(&line) {
                        utils::handle_message(message);
                    }
                }
            }
            Ok::<(), Box<dyn std::error::Error>>(())
        } => (),
        _ = async {
            loop {
                tokio::time::sleep(Duration::from_secs(43200)).await;

                let channel = String::from("#osu");
                let content = utils::generate_markov_message(channel).await;

                println!("{:#?}", content);

            }
        } => (),
    }

    Ok(())
}
