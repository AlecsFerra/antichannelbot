use std::env;
use teloxide::{dispatching::update_listeners::webhooks, prelude::*};
use url::Url;

#[tokio::main]
async fn main() {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let bot = Bot::new(&token).auto_send();

    let port: u16 = env::var("PORT")
        .expect("PORT env variable is not set")
        .parse()
        .expect("PORT env variable value is not an integer");
    println!("Port is set to: {}", port);


    let addr = ([127, 0, 0, 1], port).into();

    let host = env::var("HOST").expect("HOST env variable is not set");
    let url = Url::parse(&format!("https://{host}/webhooks/{token}")).unwrap();
    println!("Url is set to {}", url);

    let listener = webhooks::axum(bot.clone(), webhooks::Options::new(addr, url))
        .await
        .expect("Couldn't setup webhook");
    
    println!("Setup complete!");

    teloxide::repl_with_listener(
        bot,
        |message: Message, bot: AutoSend<Bot>| async move {
            if message.from().map_or(false, |u| u.is_channel()) {
                let delete = bot.delete_message(message.chat.id, message.id).await;
                if delete.is_err() {
                    bot.send_message(
                        message.chat.id,
                        "Non sono riuscito a cancellare il messaggio",
                    )
                    .await?;
                }
            }
            respond(())
        },
        listener,
    )
    .await;
}
