use dotenv::dotenv;
use futures::join;
use std::env;
use std::sync::{Arc, Mutex};

use twitter_v2::authorization::Oauth1aToken;
use twitter_v2::TwitterApi;

use rusqlite::Connection;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
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

    let write_half = Arc::new(Mutex::new(write_half));

    join!(
        handle_irc(read_half, Arc::clone(&write_half)),
        handle_twitter(),
        keep_irc_alive(Arc::clone(&write_half))
    );

    Ok(())
}

async fn handle_irc(read_half: OwnedReadHalf, write_half: Arc<Mutex<OwnedWriteHalf>>) {
    let mut reader = BufReader::new(read_half).lines();

    while let Some(line) = reader.next_line().await.unwrap() {
        if line.contains("PING") {
            // Respond to PING to keep the connection alive
            let response = line.replace("PING", "PONG");

            let mut write_half = write_half.lock().unwrap();
            if let Err(e) = write_half.write_all(response.as_bytes()).await {
                eprintln!("Failed to write response: {}", e);
            }
            println!("PROGRAM: Responded to PING");
        } else {
            if let Some(message) = utils::parse_irc_msg(&line) {
                println!("CHAT: {}", message.content);
                utils::handle_message(message);
            }
        }
    }
}

async fn keep_irc_alive(write_half: Arc<Mutex<OwnedWriteHalf>>) {
    loop {
        tokio::time::sleep(Duration::from_secs(30)).await;
        let mut write_half = write_half.lock().unwrap();
        if let Err(e) = write_half.write_all(b"PING :keepalive\r\n").await {
            eprintln!("IRC: Failed to send PING: {}", e);
        }
        println!("IRC: Sent PING");
    }
}

async fn handle_twitter() {
    let consumer_key =
        env::var("TWITTER_CONSUMER_KEY").expect("`TWITTER_CONSUMER_KEY` is not defined in .env");
    let consumer_secret = env::var("TWITTER_CONSUMER_SECRET")
        .expect("`TWITTER_CONSUMER_SECRET` is not defined in .env");
    let token =
        env::var("TWITTER_ACCESS_TOKEN").expect("`TWITTER_ACCESS_TOKEN` is not defined in .env");
    let secret =
        env::var("TWITTER_ACCESS_SECRET").expect("`TWITTER_ACCESS_SECRET` is not defined in .env");

    let auth = Oauth1aToken::new(consumer_key, consumer_secret, token, secret);
    let api = TwitterApi::new(auth);

    loop {
        println!("TWITTER: Started Twitter loop");
        // Wait 6 hours before tweeting
        const HOURS_TO_WAIT: u64 = 6;
        tokio::time::sleep(Duration::from_secs(60 * 60 * HOURS_TO_WAIT)).await;

        let channel = String::from("#osu");
        let content = utils::generate_markov_message(channel).await;
        if let Some(content) = content {
            println!("TWITTER: markov message= {content}");
            if let Err(e) = api.post_tweet().text(content).send().await {
                eprintln!("There was an error while posting tweet: {}", e);
            }
        }
    }
}
