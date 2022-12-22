use std::io::{Error, ErrorKind};
use std::str::FromStr;

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

fn main() {
    // let first_question = QuestionId("Why".to_string());
    let question = Question::new(
        QuestionId::from_str("Why").expect("No id provided"),
        "First Question".to_string(),
        "Question content".to_string(),
        Some(vec!["faq".to_string()]),
    );

    println!("{:?}.", question);
}
