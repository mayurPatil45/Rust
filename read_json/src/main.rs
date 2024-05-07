use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

fn main() {
    let json = fs::read_to_string("./test.json").expect("Failed to read file");
    let parsed: Article = read_json_typed(&json);

    println!(
        "\n\nThe name of the first paragraph is: {}",
        parsed.paragraph[0].name
    );
}

fn read_json_typed(raw_json: &str) -> Article {
    let parsed: Article = serde_json::from_str(raw_json).unwrap();
    parsed
}
