use reqwest;

#[tokio::main]
pub async fn get_all_heros() {
    let response = reqwest::get("https://api.opendota.com/api/heroes")
    .await
    .unwrap()
    .text()
    .await;
    println!("Heros list: {:?}", response);
}

#[tokio::main]
pub async fn get_teams() {
    let response = reqwest::get("https://api.opendota.com/api/teams")
    .await
    .unwrap()
    .text()
    .await;
    println!("Heros list: {:?}", response);
}