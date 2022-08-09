use reqwest;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Hero {
    id: i32,
    name: String,
    localized_name: String,
    primary_attr: String,
    attack_type: String,
    roles: Vec<String>,
    legs: i32
}

#[tokio::main]
pub async fn get_all_heros() {
    let mut response = reqwest::get("https://api.opendota.com/api/heroes").await.unwrap();

    let json_response: Vec<Hero> = response.json().await.unwrap();
    
    for hero in json_response {
        println!("Hero local name :{:?}, Hero roles:{:?}", hero.localized_name, hero.roles);
    }
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