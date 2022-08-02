struct Question {   // Compiler warning that Question is never constructed - expected
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}
struct QuestionId(String);  // Compiler warning that QuestionId is never consctucted - expected

impl Question { // Compiler warning that Question::new is never used - expected
    fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Question {
            id,
            title,
            content,
            tags,
        }
    }
}

fn main() {
}
