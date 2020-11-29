use std::env;
use log::info;
use futures::StreamExt;
use telegram_bot::*;

mod bot;

struct Model {
    response_type: String,
    text: String
}

// struct BotResponse {
//     resp: Vec<Model>
// }

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
                    for resp in watson.as_array().unwrap() {
                        // api.send(message.from.text(format!("Oi {}, Você escreveu: '{}'", &message.from.first_name, data))).await?;
                        api.send(
                            message.from.text(
                                resp["text"].as_str().unwrap()
                            )
                        ).await?;
                        println!("{}", resp);
                    }
                    // println!("{:#?}", watson);
            }
        }
    }
    Ok(())
}