use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Text {
    text: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Payload {
    input: Text
}

// #[tokio::main]
pub async fn chat() /*-> Result<(), dyn std::error::Error>*/ {
    let watson_url = std::env::var("WATSON_URL").expect("No WATSON_URL provided");
    let watson_apikey = std::env::var("WATSON_APIKEY").expect("No WATSON_APIKEY provided");
    let assistant_id = std::env::var("WATSON_ASSISTANT_ID").expect("No WATSON_ASSISTANT_ID provided");
    let url = format!("{}/v2/assistants/{}/message?version=2020-04-01", &watson_url, &assistant_id);
    println!("{}", &url);

    let text = Text {
        text: String::from("Quando a fundação foi fundanda")
    };

    let payload = Payload {
        input: text
    };


    println!("{}", &watson_url);
    println!("{}", &watson_apikey);
    println!("{}", &assistant_id);
    let resp = reqwest::Client::new()
        .post(&url)
        .basic_auth("apikey", Some(watson_apikey))
        .json(&payload)
        .send()
        .await;
        // .body()
        // .json()
        // .await
        // .unwrap();
        // .json::<HashMap<String, String>>()
        // .await?;
    // println!("{:#?}", resp);
    match resp {
        Ok(response) => {
            // let r = response.json::<HashMap<String, String>>().await;
            // let r = response.json::<HashMap<String, String>>().await;
            println!("{:#?}", response);
        },
        Err(error) => {
            println!("NO");
        }

    }
    // Ok(())
}

// pub fn chat2() {
//     let watson_url = std::env::var("WATSON_URL").expect("No WATSON_URL provided");
//     let watson_apikey = std::env::var("WATSON_APIKEY").expect("No WATSON_APIKEY provided");
//     let assistant_id = std::env::var("WATSON_ASSISTANT_ID").expect("No WATSON_ASSISTANT_ID provided");
//     let url = format!("{}/v2/assistants/{}/message?version=2020-04-01", &watson_url, &assistant_id);

//     let text = Text {
//         text: String::from("Quando a fundação foi fundanda")
//     };

//     let payload = Payload {
//         input: text
//     };

//     let resp = reqwest::Client::new()
//         .get("https://httpbin.org/ip")
        // .post(&url)
        // .basic_auth("apikey", Some(watson_apikey))
        // .json(&payload)
        // .send()
        // .json::<HashMap<String, String>>()?;
        // println!("{:#?}", resp);
// }