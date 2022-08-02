use std::io::{Error, ErrorKind};
use std::str::FromStr;

#[derive(Debug)]
struct QuestionId(String);

impl FromStr for QuestionId {
    type Err = Error;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(QuestionId(id.to_string())),
            // removing or making the following line a comment will demonstrate the error in
            // Listing 1.9
            true => Err(Error::new(ErrorKind::InvalidInput, "No id provided")),
        }
    }
}

fn main() {
    let question_id = QuestionId::from_str("1").expect("No id provider");
    println!("QuestionId is: {:?}", question_id);
}
