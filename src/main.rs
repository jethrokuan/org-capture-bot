use std::env;

use tokio::stream::StreamExt;
use telegram_bot::*;
use std::path::PathBuf;
use std::io::Write;
use std::fs::OpenOptions;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let file_path = env::var("INBOX_FILE_PATH").expect("INBOX_FILE_PATH not set");
    let api = Api::new(token);
    

    // Fetch new updates via long poll method
    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        // If the received update contains a new message...
        let update = update?;
        if let UpdateKind::Message(message) = update.kind {
            if let MessageKind::Text { ref data, .. } = message.kind {
                let file = PathBuf::from(&file_path);
                let file = OpenOptions::new().append(true).open(file).expect("Could not open file");
                match write!(&file, "* TODO {}\n", data) {
                    Ok(_) => api.send(message.text_reply("Saved!")).await?,   
                    Err(e) => api.send(message.text_reply(format!("{}", e).to_string().as_str())).await?
                };
            }
        }
    }
    Ok(())
}
