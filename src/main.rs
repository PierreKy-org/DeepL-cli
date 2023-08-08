
use reqwest::{self, header::{USER_AGENT, AUTHORIZATION, CONTENT_TYPE}, Response};
use std::env;   
use dotenv::dotenv;

async fn send_request_to_deepl(text : &String, dest_language : &String) -> Response {

    let body_request = "{\"text\":[\"".to_string()+ text + "\"],\"target_lang\":\"" + dest_language + "\"}";

    let token = "DeepL-Auth-Key ".to_string() + &env::var("DEEPL_TOKEN").unwrap();

    let client = reqwest::Client::new();

    let response = client
    .post("https://api-free.deepl.com/v2/translate")
    .body(body_request)
    .header(AUTHORIZATION, token)
    .header(USER_AGENT, "cli/0.0.1")
    .header(CONTENT_TYPE, "application/json")
    .send()
    .await;
    response.unwrap()
}
#[tokio::main]
async fn main() {
    dotenv().ok();
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Please provide a text to translate and a destination language.\nExample : ./deepl-cli \"Hello World\" FR");
        return;
    }
    //check if DEEPL_TOKEN exist 
    match env::var("DEEPL_TOKEN") {
        Ok(token) => {
            if token == "" {
                println!("Please provide a valid DEEPL_TOKEN in your .env file.");
                return;
            }
        }
        Err(_) => {
            println!("Please provide a valid DEEPL_TOKEN in your .env file.");
            return;
        }
    }
    let response = send_request_to_deepl(&args[1], &args[2]).await;

    match response.status() {
        reqwest::StatusCode::OK => {
            println!("Success! {:?}", response.json::<serde_json::Value>().await.unwrap());
        }
        reqwest::StatusCode::FORBIDDEN => {
            println!("Token is invalid! Please check your .env file and try again.");
        }
        other => {
            panic!("Oh no ! Something unexpected happened. Code Error : {:?}", other);
        }
    };

}


