use std::{collections::HashMap, time::Duration};

use serde::Deserialize;
use serde_json::{json, Value};
use tokio::sync::mpsc::Sender;

use crate::{models::ComponentInfo, database::last_api_key};

#[derive(Deserialize, Debug)]
struct Token {
    data: String,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[allow(unused, non_snake_case)]
struct Data {
    componentInfos: Option<Vec<ComponentInfo>>,
    lastKey: String,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
struct ComponentInfoResponse {
    success: bool,
    code: i32,
    data: Option<Data>,
    message: Option<String>,
}

pub async fn pull_from_api(sender: Sender<Vec<ComponentInfo>>) {
    let client = reqwest::Client::new();

    let token = client
        .post("https://jlcpcb.com/external/genToken")
        .json(&json!({
            "appKey": "appKey2488994",
            "appSecret": "appSecret2488994",
        }))
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json::<Token>()
        .await
        .unwrap();

    let mut last_key = last_api_key();
    dbg!(&last_key);

    loop {
        let mut builder = client.post("https://jlcpcb.com/external/component/getComponentInfos");
        if let Some(last_key) = &last_key {
            let form = HashMap::from([("lastKey", last_key)]);
            builder = builder.form(&form);
        }
        let resp = builder
            .header("externalApiToken", &token.data)
            .send()
            .await
            .unwrap()
            .error_for_status()
            .unwrap();

        // dbg!(&resp.text().await.unwrap()[..150]);
        // panic!("");

        let resp = resp.json::<ComponentInfoResponse>().await.unwrap();

        if let Some(data) = resp.data {
            match data.componentInfos {
                Some(mut components) => {
                    components.iter_mut().for_each(|component|{
                        component.api_last_key = last_key.clone();
                    });
                    sender.send(components).await.unwrap();
                }
                None => break,
            };
            last_key = Some(data.lastKey);
        } else {
            if resp.code == 429 {
                std::thread::sleep(Duration::from_secs_f32(2.));
            } else {
                panic!("unexpected state");
            }
        }
        std::thread::sleep(Duration::from_millis(250));
    }
}
