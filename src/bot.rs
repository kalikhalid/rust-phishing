use teloxide::prelude::*;
use teloxide::types::Recipient;

pub async fn bob(text: String) {
    log::info!("Starting throw dice bot...");

    let bot = Bot::from_env();

    let user_id = Recipient::from("807123271".to_string());
    bot.send_message(user_id, text).await.unwrap();
}