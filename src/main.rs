#[derive(Debug)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}
#[derive(Debug)]
struct QuestionId(String);

impl Question {
    fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Self {
            id,
            title,
            content,
            tags,
        }
    }
}

fn main() {
    let first_question = QuestionId("Why".to_string());
    let question = Question::new(
        first_question,
        "First Question".to_string(),
        "Question content".to_string(),
        Some(vec!["faq".to_string()]),
    );

    println!(
        "{:?}, {}, {}, {:?}.",
        question.id.0, question.title, question.content, question.tags.unwrap()[0]
    );
}
