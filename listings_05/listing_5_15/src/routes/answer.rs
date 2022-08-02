use std::collections::HashMap;
use warp::{
    http::StatusCode,
    Reply,
    Rejection,
};

use crate::{
    types::{
        answer::{
            Answer,
            AnswerId,
        },
        question::QuestionId,
    },
    store::Store,
};


pub async fn add_answer(
    store: Store,
    params: HashMap<String, String>,
) -> Result<impl Reply, Rejection> {
    let answer = Answer {
        id: AnswerId("1".to_string()),
        content: params.get("content").unwrap().to_string(),
        question_id: QuestionId(params.get("questionId").unwrap().to_string()),
    };

    store.answers.write().insert(answer.id.clone(), answer);

    Ok(warp::reply::with_status("Answer added", StatusCode::OK))
}