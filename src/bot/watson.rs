use serde::{Deserialize, Serialize};
use serde_json;
use log;

#[derive(Debug, Serialize, Deserialize)]
struct Text {
    text: String
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    user_id: String
}

#[derive(Debug, Serialize, Deserialize)]
struct System {
    system: User
}

#[derive(Debug, Serialize, Deserialize)]
struct Context {
    global: System
    // skills: Some()
}

#[derive(Debug, Serialize, Deserialize)]
struct Payload {
    input: Text,
    // context: Context
}

#[derive(Debug, Serialize, Deserialize)]
struct Session {
    session_id: String
}

struct TelegramUser {
    chat_id: String,
    session_id: String
}

// #[tokio::main]
pub async fn chat(input: &str) -> Result<serde_json::Value, reqwest::Error> {
    log::info!("Requesting Watson Watson API");
    let watson_url = std::env::var("WATSON_URL").expect("No WATSON_URL provided");
    let watson_apikey = std::env::var("WATSON_APIKEY").expect("No WATSON_APIKEY provided");
    let assistant_id = std::env::var("WATSON_ASSISTANT_ID").expect("No WATSON_ASSISTANT_ID provided");

    // Stateless chat with Watson
    let url = format!("{}/v2/assistants/{}/message?version=2020-04-01", &watson_url, &assistant_id);

    let text = Text {
        text: String::from(input)
    };

    let payload = Payload {
        input: text
    };

    let resp = reqwest::Client::new()
        .post(&url)
        .basic_auth("apikey", Some(watson_apikey))
        .json(&payload)
        .send()
        .await;

    // println!("{:#?}", resp);
    match resp {
        Ok(response) => {
            let body = response.json::<serde_json::Value>().await.unwrap();
            match body.get("output") {
                Some(value) => {
                    match value.get("generic") {
                        Some(assistant_resp) => {
                            log::info!("API request successful");
                            return Ok(serde_json::to_value(assistant_resp).unwrap())
                        },
                        None => {
                            log::warn!("Virtual assistant does not find any answer");
                            let ret = r#"[{
                                "text": "Não foi possível se conectar ao bot"
                            }]"#;
                            return Ok(serde_json::from_str(&ret).unwrap());
                        }
                    }
                },
                None => {
                    log::warn!("Virtual assistant does not find any answer");
                    let ret = r#"[{
                        "text": "Não foi possível se conectar ao bot"
                    }]"#;
                    return Ok(serde_json::from_str(&ret).unwrap());
                }
            };
        },
        Err(error) => {
            log::error!("API request not successfull: {}", error);
            let data = r#"[{
                "text": "Não foi possível se conectar ao bot"
            }]"#;
            Ok(serde_json::from_str(&data).unwrap())
        }
    }
}

pub async fn chat_statefull(input: &str, session_id: String) -> Result<serde_json::Value, reqwest::Error> {
    log::info!("Requesting Watson Watson API");
    let watson_url = std::env::var("WATSON_URL").expect("No WATSON_URL provided");
    let watson_apikey = std::env::var("WATSON_APIKEY").expect("No WATSON_APIKEY provided");
    let assistant_id = std::env::var("WATSON_ASSISTANT_ID").expect("No WATSON_ASSISTANT_ID provided");

    // Stafull chat with Watson
    let url = format!("{}/v2/assistants/{}/sessions/{}/message?version=2020-04-01", &watson_url, &assistant_id, &session_id);

    let text = Text {
        text: String::from(input)
    };

    let payload = Payload {
        input: text
    };

    let resp = reqwest::Client::new()
        .post(&url)
        .basic_auth("apikey", Some(watson_apikey))
        .json(&payload)
        .send()
        .await;

    match resp {
        Ok(response) => {
            let body = response.json::<serde_json::Value>().await.unwrap();
            match body.get("output") {
                Some(value) => {
                    match value.get("generic") {
                        Some(assistant_resp) => {
                            log::info!("API request successful");
                            return Ok(serde_json::to_value(assistant_resp).unwrap())
                        },
                        None => {
                            log::warn!("Virtual assistant does not find any answer");
                            let ret = r#"[{
                                "text": "Não foi possível se conectar ao bot"
                            }]"#;
                            return Ok(serde_json::from_str(&ret).unwrap());
                        }
                    }
                },
                None => {
                    log::warn!("Virtual assistant does not find any answer");
                    let ret = r#"[{
                        "text": "Não foi possível se conectar ao bot"
                    }]"#;
                    return Ok(serde_json::from_str(&ret).unwrap());
                }
            };
        },
        Err(error) => {
            log::error!("API request not successfull: {}", error);
            let data = r#"[{
                "text": "Não foi possível se conectar ao bot"
            }]"#;
            Ok(serde_json::from_str(&data).unwrap())
        }
    }
}

// At the moment the chat conversation will be only statless
pub async fn create_session() -> Result<String, reqwest::Error> {
    let watson_url = std::env::var("WATSON_URL").expect("No WATSON_URL provided");
    let watson_apikey = std::env::var("WATSON_APIKEY").expect("No WATSON_APIKEY provided");
    let assistant_id = std::env::var("WATSON_ASSISTANT_ID").expect("No WATSON_ASSISTANT_ID provided");
    let url = format!("{}/v2/assistants/{}/sessions?version=2020-04-01", &watson_url, &assistant_id);

    let resp = reqwest::Client::new()
        .post(&url)
        .basic_auth("apikey", Some(watson_apikey))
        .send()
        .await;

    match resp {
        Ok(raw) => {
            let body: Session = raw.json::<Session>().await.unwrap();
            Ok(body.session_id)
        },
        Err(http_error) => {
            println!("Not possible to make the request: {}", http_error);
            Ok("".to_string())
        }
    }
}