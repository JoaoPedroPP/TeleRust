use std::env;
use log;
use futures::StreamExt;
use telegram_bot::*;

mod bot;

#[tokio::main]
async fn main() -> Result<(), Error> {
    log::info!("App started");
    // load env values
    dotenv::dotenv().ok();

    // initialize logger
    env_logger::init();
    // log::set_logger(env_logger).unwrap();
    log::set_max_level(log::LevelFilter::Trace);

        
        let token = env::var("TELEGRAM_TOKEN").expect("TELEGRAM_TOKEN not set");
        let api = Api::new(token);
        
        let mut stream = api.stream();
        while let Some(update) = stream.next().await {
            let update = update?;
            if let UpdateKind::Message(message) = update.kind {
                if let MessageKind::Text { ref data, .. } = message.kind {
                    let t = chrono::Utc::now();
                    log::info!("New Message from -> <{}>: {}", &message.from.first_name, data);
                    let watson = bot::chat().await.unwrap();
                    for resp in watson.as_array().unwrap() {
                        api.send(
                            message.from.text(
                                resp["text"].as_str().unwrap()
                            )
                        ).await?;
                    }
            }
        }
    }
    Ok(())
}