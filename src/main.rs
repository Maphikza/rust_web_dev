use std::io::{Error, ErrorKind};
use std::str::FromStr;

use warp::Filter;

#[derive(Debug)]
struct Question {
    _id: QuestionId,
    _title: String,
    _content: String,
    _tags: Option<Vec<String>>,
}
#[derive(Debug)]
struct QuestionId(String);

impl Question {
    fn new(_id: QuestionId, _title: String, _content: String, _tags: Option<Vec<String>>) -> Self {
        Question {
            _id,
            _title,
            _content,
            _tags,
        }
    }
}

impl FromStr for QuestionId {
    type Err = std::io::Error;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(QuestionId(id.to_string())),
            true => Err(Error::new(ErrorKind::InvalidInput, "No id provided")),
        }
    }
}

#[tokio::main]
async fn main() {
    let hello = warp::get().map(|| format!("Hello World"));

    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}
