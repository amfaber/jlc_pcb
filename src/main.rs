use jlc_pcb::{
    api_pull::pull_from_api,
    database::{get_components, insert_components},
};
use tokio::sync::mpsc::channel;

#[tokio::main]
async fn main() {
    let (sender, receiver) = channel(100);
    tokio::spawn(pull_from_api(sender));
    insert_components(receiver).await;

    dbg!(get_components());
    dbg!(get_components().len());
}
