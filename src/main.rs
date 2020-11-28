use std::env;
use log::info;
use futures::StreamExt;
use telegram_bot::*;

mod bot;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // load env values
    dotenv::dotenv().ok();

    // initialize logger
    env_logger::builder()
        .format_timestamp(None)
        .init();

        
        let token = env::var("TELEGRAM_TOKEN").expect("TELEGRAM_TOKEN not set");
        // let t = chrono::Utc::now();
        let api = Api::new(token);
        
        let mut stream = api.stream();
        while let Some(update) = stream.next().await {
            let update = update?;
            if let UpdateKind::Message(message) = update.kind {
                if let MessageKind::Text { ref data, .. } = message.kind {
                    let t = chrono::Utc::now();
                    // println!("{} - Message from -> <{}>: {}", &t, &message.from.first_name, data);
                    info!("{} - Message from -> <{}>: {}", &t, &message.from.first_name, data);
                    let watson = bot::chat().await.unwrap();
                    println!("{:#?}", watson);
                api.send(message.from.text(format!("Oi {}, Você escreveu: '{}'", &message.from.first_name, data))).await?;
            }
        }
    }
    Ok(())
}