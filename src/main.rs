use std::time::Instant;

use jlc_pcb::{
    api_pull::{pull_from_api, ComponentInfoResponse},
    database::{get_components, insert_components},
    models::ComponentInfo,
};
use tokio::sync::mpsc::channel;

async fn continue_pull(){
    let (sender, receiver) = channel(100);
    tokio::spawn(pull_from_api(sender));
    insert_components(receiver).await;
}

#[tokio::main]
async fn main() {
    let now = Instant::now();
    let components = get_components(Some("trans"));
    let elapsed = now.elapsed();

    dbg!(&components);

    dbg!(components.len());
    dbg!(elapsed);
    
}
