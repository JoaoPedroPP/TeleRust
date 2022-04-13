use std::env;
use log;
use futures::StreamExt;
use std::collections::HashMap;
use telegram_bot::*;

mod bot;

#[tokio::main]
async fn main() -> Result<(), Error> {
    log::info!("App started");
    // load env values
    dotenv::dotenv().ok();

    // initialize logger
    env_logger::init();

    let mut users = HashMap::new();
    // search_user(&users, 1000).await;
    // insert_user(&mut users, 1000).await;
    // search_user(&users, 1000).await;

        
    let token = env::var("TELEGRAM_TOKEN").expect("TELEGRAM_TOKEN not set");
    let api = Api::new(token);
    
    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        let update = update?;
        if let UpdateKind::Message(message) = update.kind {
            if let MessageKind::Text { ref data, .. } = message.kind {
                log::info!("New Message from -> <{}>: {}", &message.from.first_name, data);
                let telegram_user: bot::TelegramUser = match bot::search_user(&users, message.from.id.into()).await {
                    bot::User::Found(user) => {
                        user
                    },
                    bot::User::NoRecord => {
                        log::info!("User {} not found, creating a new record", message.from.id);
                        log::info!("Creating first session for {}", message.from.id);
                        let new_session = bot::create_session().await.unwrap();
                        bot::insert_user(&mut users, message.from.id.into(), new_session).await
                    },
                };
                // println!("{}", (chrono::Utc::now().timestamp() - telegram_user.last_interaction));
                if (chrono::Utc::now().timestamp() - telegram_user.last_interaction) > 280 {
                    // Sessão expirou e usuário prcisa de uma nova sessão
                    // println!("Cria sessão. Tempo expirado.");
                    log::info!("Session timed out, creating new one for {}", message.from.id);
                    let new_session = bot::create_session().await.unwrap();
                    bot::update_user_session(&mut users, message.from.id.into(), new_session).await;
                }
                // let watson = bot::chat(data).await.unwrap();
                let watson = bot::chat_statefull(data, telegram_user.session_id).await.unwrap();
                for resp in watson.as_array().unwrap() {
                    log::info!("Answering the chat for {}", message.from.id);
                    if resp["response_type"].as_str().unwrap() == "text" {
                        api.send(
                            message.from.text(
                                resp["text"].as_str().unwrap()
                            )
                        ).await?;
                    } else if resp["response_type"].as_str().unwrap() == "option" {
                        let title = if resp["title"].as_str() != None {
                            resp["title"].as_str().unwrap()
                        } else {
                            "Escolha uma das opções abaixo"
                        };
                        let mut keyboard = ReplyKeyboardMarkup::new();
                        for i in resp["options"].as_array().unwrap() {
                            let row = keyboard.add_empty_row();
                            row.push(KeyboardButton::new(i["label"].as_str().unwrap()));
                        }
                        api.send(
                            message.from.text(
                                title
                            ).reply_markup(keyboard)
                        ).await?;
                    } else if resp["response_type"].as_str().unwrap() == "image" {
                        let source = resp["source"].as_str().unwrap();
                        if resp["title"].as_str() != None {
                            api.send(
                                message.from.photo(
                                    InputFileRef::new(source)
                                ).caption(resp["title"].as_str().unwrap())
                            ).await?;
                        } else{
                            api.send(
                                message.from.photo(
                                    InputFileRef::new(source)
                                )
                            ).await?;
                        }
                    } else {
                        api.send(
                            message.from.text(
                                "Else"
                            )
                        ).await?;
                    }
                }
                bot::update_user_last_iterarion(&mut users, message.from.id.into()).await;
                log::info!("Last interaction updated for user {}", message.from.id);
            }
        }
    }
    Ok(())
}
