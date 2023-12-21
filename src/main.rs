use std::collections::HashMap;

use serde::Deserialize;
use serde_json::{json, Map, Value};

#[derive(Deserialize, Debug)]
struct Token {
    data: String,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[allow(unused)]
struct ComponentInfos {
    lcscPart: String,
    firstCategory: String,
    secondCategory: String,
    mfrPart: String,
    solderJoint: String,
    manufacturer: String,
    libraryType: String,
    description: String,
    datasheet: String,
    price: String,
    stock: i32,
    package: String,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
struct Data {
    componentInfos: Vec<ComponentInfos>,
    lastKey: String,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
struct ComponentInfoResponse {
    success: bool,
    code: i32,
    data: Option<Data>,
}

#[tokio::main]
async fn main() {
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
        .json::<Token>()
        .await
        .unwrap();

    let mut last_key = None;

    loop {
        let mut builder = client.post("https://jlcpcb.com/external/component/getComponentInfos");
        if let Some(last_key) = last_key {
            let form = HashMap::from([("lastKey", last_key)]);
            builder = builder.form(&form);
        }
        let resp = builder
            .header("externalApiToken", &token.data)
            .send()
            .await
            .unwrap()
            .json::<ComponentInfoResponse>()
            .await
            .unwrap();

        if let Some(data) = resp.data{
            last_key = Some(data.lastKey);
            dbg!(data.componentInfos.len());
        } else {
            break
        }
    }
}
