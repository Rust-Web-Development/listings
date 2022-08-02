struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

struct QuestionId(String);

impl Question {
    fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Question {
            id,
            title,
            content,
            tags,
        }
    }

    fn update_question_title(&self, new_title: String) -> Self {
        new(self.id, new_title, self.content, self.tags)
    }
}

fn main() {
    let mut question = Question::new("1", "First Question", "Content of question", ["faq"]);
    println!("{}", question);
    question = question.update_question_title("Initial Question");
    println!("{}", question);
}
