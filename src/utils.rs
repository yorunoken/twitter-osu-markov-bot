use rand::Rng;
use rusqlite::{params, Connection};

use crate::{markov_chain, Message};

pub async fn generate_markov_message(channel: String) -> Option<String> {
    const DATABASE_MESSAGE_FETCH_LIMIT: usize = 2000;

    let sentences: Vec<String> = tokio::task::spawn_blocking(move || {
        let conn = Connection::open("data.db").expect("Unable to open database");

        let mut stmt = conn
            .prepare("SELECT content FROM messages WHERE channel = ?1 ORDER BY RANDOM() LIMIT ?2;")
            .unwrap();

        let sentences_iter = stmt
            .query_map(params![channel, DATABASE_MESSAGE_FETCH_LIMIT], |row| {
                row.get(0)
            })
            .unwrap();

        sentences_iter.map(|result| result.unwrap()).collect()
    })
    .await
    .unwrap();

    // I feel like this should be the least amount of data present
    // before the bot finally stops just repeating sentences.
    if sentences.len() < 500 {
        return None;
    }

    let mut rng = rand::thread_rng();

    let mut markov_chain = markov_chain::Chain::new();
    markov_chain.train(sentences);

    let max_words = rng.gen_range(1..15);
    let content = markov_chain.generate(max_words);
    Some(content)
}

pub fn handle_message(message: Message) {
    let conn = Connection::open("data.db").expect("Unable to open database.");

    conn.execute(
        "INSERT INTO messages (content, nickname, channel) VALUES (?1, ?2, ?3)",
        params![message.content, message.nickname, message.channel],
    )
    .expect("Failed to insert word into database");
}
