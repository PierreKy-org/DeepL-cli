#[macro_use] extern crate prettytable;
use prettytable::Table;
use reqwest::{self, header::{USER_AGENT, AUTHORIZATION, CONTENT_TYPE}, Response};
use std::env;   
use dotenv::dotenv;
use colored::Colorize;

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


fn print_json_cli(json : serde_json::Value, text_before : &String, dest_language : &String) {
    //text before 

    let source = format!("Source : {}", json["translations"][0]["detected_source_language"].as_str().unwrap().red());
    let text_before = format!("{}", text_before.red());
    let destination = format!("Destination : {}", dest_language.green());
    let translation = format!("{}", json["translations"][0]["text"].as_str().unwrap().green().bold());


    let mut table = Table::new();

    // Add a row per time
    table.add_row(row![source, destination]);
    table.add_row(row![text_before, translation]);
    // Print the table to stdout
    table.printstd();
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
            print_json_cli(response.json::<serde_json::Value>().await.unwrap(), &args[1], &args[2]);
        }
        reqwest::StatusCode::FORBIDDEN => {
            println!("Token is invalid! Please check your .env file and try again.");
        }
        other => {
            panic!("Oh no ! Something unexpected happened. Code Error : {:?}", other);
        }
    };

}


