use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, name = "deepl-cli")]
pub struct Cli {
    /// The text to translate
    pub text: String,
    /// The language to translate to
    pub language: String,
}