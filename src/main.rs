use std::env;
use log;
use futures::StreamExt;
use std::collections::HashMap;
use telegram_bot::*;

mod bot;


#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct TelegramUser {
    session_id: String,
    last_interaction: i64
}

impl TelegramUser {
    fn new (session_id: &str, last_interaction: i64) -> TelegramUser {
        TelegramUser { session_id: session_id.to_string(), last_interaction: last_interaction }
    }
}

enum User {
    Found(TelegramUser),
    NoRecord
}

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
                let telegram_user: TelegramUser = match search_user(&users, message.from.id.into()).await {
                    User::Found(user) => {
                        println!("User: {:?}", user);
                        user
                    },
                    User::NoRecord => {
                        println!("Não achei. Criando novo usuário");
                        insert_user(&mut users, message.from.id.into()).await
                    },
                };
                println!("{}", (chrono::Utc::now().timestamp() - telegram_user.last_interaction));
                if telegram_user.session_id == "" {
                    // primeira iteração do usuario com o bot. Criar sessão.
                    println!("Cria sessão. Primeira sessão");
                    let new_session = bot::create_session().await.unwrap();
                    update_user_session(&mut users, message.from.id.into(), new_session).await;
                }
                else if (chrono::Utc::now().timestamp() - telegram_user.last_interaction) > 280 {
                    // Sessão expirou e usuário prcisa de uma nova sessão
                    println!("Cria sessão. Tempo expirado.");
                    let new_session = bot::create_session().await.unwrap();
                    update_user_session(&mut users, message.from.id.into(), new_session).await;
                }
                let watson = bot::chat(data).await.unwrap();
                // let watson = bot::chat_statefull(data, telegram_user.session_id).await.unwrap();
                for resp in watson.as_array().unwrap() {
                    log::info!("Answering the chat");
                    api.send(
                        message.from.text(
                            resp["text"].as_str().unwrap()
                        )
                    ).await?;
                }
                update_user_last_iterarion(&mut users, message.from.id.into()).await;
            }
        }
    }
    Ok(())
}

async fn search_user(users: &HashMap<i64, TelegramUser>, id: i64) -> User {
    for (chat_id, user) in users {
        println!("User: {:?}, chat_id: {}", user, chat_id);
        if id == *chat_id {
            return User::Found(user.clone());
        }
    }
    return User::NoRecord;
}

async fn insert_user(users: &mut HashMap<i64, TelegramUser>, id: i64) -> TelegramUser {
    let t = chrono::Utc::now().timestamp();
    users.insert(id, TelegramUser::new("", t));
    return TelegramUser { session_id: String::from(""), last_interaction: t };
}

async fn update_user_session(users: &mut HashMap<i64, TelegramUser>, id: i64, update_session: String) {
    for (chat_id, user) in users {
        println!("User: {:?}, chat_id: {}", user, chat_id);
        if id == *chat_id {
            user.session_id = update_session.to_string();
        }
    }
}

async fn update_user_last_iterarion(users: &mut HashMap<i64, TelegramUser>, id: i64) {
    for (chat_id, user) in users {
        println!("User: {:?}, chat_id: {}", user, chat_id);
        if id == *chat_id {
            user.last_interaction = chrono::Utc::now().timestamp();
        }
    }
}