use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct TelegramUser {
    pub session_id: String,
    pub last_interaction: i64
}

impl TelegramUser {
    fn new (session_id: &str, last_interaction: i64) -> TelegramUser {
        TelegramUser { session_id: session_id.to_string(), last_interaction: last_interaction }
    }
}

pub enum User {
    Found(TelegramUser),
    NoRecord
}

pub async fn search_user(users: &HashMap<i64, TelegramUser>, id: i64) -> User {
    for (chat_id, user) in users {
        // println!("User: {:?}, chat_id: {}", user, chat_id);
        if id == *chat_id {
            return User::Found(user.clone());
        }
    }
    return User::NoRecord;
}

pub async fn insert_user(users: &mut HashMap<i64, TelegramUser>, id: i64, session: String) -> TelegramUser {
    let t = chrono::Utc::now().timestamp();
    users.insert(id, TelegramUser::new(session.as_str(), t));
    return TelegramUser { session_id: session, last_interaction: t };
}

pub async fn update_user_session(users: &mut HashMap<i64, TelegramUser>, id: i64, update_session: String) {
    for (chat_id, user) in users {
        // println!("User: {:?}, chat_id: {}", user, chat_id);
        if id == *chat_id {
            user.session_id = update_session.to_string();
        }
    }
}

pub async fn update_user_last_iterarion(users: &mut HashMap<i64, TelegramUser>, id: i64) {
    for (chat_id, user) in users {
        // println!("User: {:?}, chat_id: {}", user, chat_id);
        if id == *chat_id {
            user.last_interaction = chrono::Utc::now().timestamp();
        }
    }
}