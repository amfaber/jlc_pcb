use std::{
    collections::{HashMap, HashSet},
    time::Duration,
};

use reqwest_middleware::ClientBuilder;
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use serde::Deserialize;
use serde_json::{json, Map, Value};

#[derive(Deserialize, Debug)]
struct Token {
    data: String,
}

#[derive(Deserialize, Debug, PartialEq, Eq, Hash)]
#[allow(unused)]
struct ComponentInfos {
    lcscPart: Option<String>,
    firstCategory: Option<String>,
    secondCategory: Option<String>,
    mfrPart: Option<String>,
    solderJoint: Option<String>,
    manufacturer: Option<String>,
    libraryType: Option<String>,
    description: Option<String>,
    datasheet: Option<String>,
    price: Option<String>,
    stock: Option<i32>,
    package: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
struct Data {
    componentInfos: Option<Vec<ComponentInfos>>,
    lastKey: String,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
struct ComponentInfoResponse {
    success: bool,
    code: i32,
    data: Option<Data>,
    message: Option<String>,
}

#[tokio::main]
async fn main() {
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(20);
    let client = ClientBuilder::new(reqwest::Client::new())
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();

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

    let mut last_key = None;

    let mut set = HashSet::new();

    let sleep = 2.;
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

        let json = resp.json::<Value>().await.unwrap();

        let resp = serde_json::from_value::<ComponentInfoResponse>(json.clone()).unwrap();

        if let Some(data) = resp.data {
            last_key = Some(data.lastKey);
            match data.componentInfos {
                Some(components) => {
                    let len = set.len();
                    set.extend(components.into_iter().map(|val| val.lcscPart.unwrap()));
                    dbg!(set.len() - len);
                    dbg!(set.len());
                }
                None => break,
            };
            // dbg!(data.componentInfos.len());
        } else {
            if resp.code == 429 {
                std::thread::sleep(Duration::from_secs_f32(sleep));
                // sleep *= 1.3;
            }
            dbg!(json);
        }
        std::thread::sleep(Duration::from_millis(250));
    }
}
