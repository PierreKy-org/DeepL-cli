mod args;

extern crate term_size; 
extern crate text_box;

use reqwest::{self, header::{USER_AGENT, AUTHORIZATION, CONTENT_TYPE}, Response};
use std::env;   
use dotenv::dotenv;
use colored::Colorize;
use text_box::TextBox;
use text_box::utils::{clear_screen, goto};
use args::Cli;
use clap::Parser;


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


fn print_result_cli(json : serde_json::Value, text_before : &String, dest_language : &String) {
    //text before 

    let source = format!("Source : {}", json["translations"][0]["detected_source_language"].as_str().unwrap().red());
    let text_before = format!("{}", text_before.red());
    let destination = format!("Destination : {}", dest_language.green());
    let translation = format!("{}", json["translations"][0]["text"].as_str().unwrap().green().bold());

    let terminal_x = (term_size::dimensions().unwrap().0) as u8;
    let terminal_y = (term_size::dimensions().unwrap().1 / 2) as u8;

    let box_source = TextBox::new(
		1, 2,
		terminal_x , terminal_y ,
		0,
		source.as_str(),
		text_before.as_str(),
	).unwrap();
    //Create TextBox
    let box_destination = TextBox::new(
		1, 10,
		terminal_x , terminal_y ,
		0,
		destination.as_str(),
		translation.as_str(),
	).unwrap();
    println!("{} {}", box_source,  box_destination);
    goto(1,40);

}




#[tokio::main]
async fn main() {
    clear_screen();
    
    dotenv().ok();

    let cli = Cli::parse();

    //check if DEEPL_TOKEN exist 
    match env::var("DEEPL_TOKEN") {
        Ok(token) => {
            if token == "" {
                println!("Please recompile with a valid DEEPL_TOKEN in your .env file.");
                return;
            }
        }
        Err(_) => {
            println!("Please recompile with a valid DEEPL_TOKEN in your .env file.");
            return;
        }
    }
    let response = send_request_to_deepl(&cli.text, &cli.language).await;

    match response.status() {
        reqwest::StatusCode::OK => {
            print_result_cli(response.json::<serde_json::Value>().await.unwrap(), &cli.text, &cli.language);
        }
        reqwest::StatusCode::FORBIDDEN => {
            println!("Token is invalid! Please check your .env file and try again.");
        }
        other => {
            panic!("Oh no ! Something unexpected happened. Code Error : {:?}", other);
        }
    };

}


