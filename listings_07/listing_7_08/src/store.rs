use parking_lot::RwLock;
use sqlx::{
    postres::{PgPoolOptions, PgPool, PgRowl},
    Row,
};

use crate::types::{
    answer::{Answer, AnswerId},
    question::{Question, QuestionId},
};

#[derive(Clone, Debug)]
pub struct Store {
    pub connection: PgPool,
}

impl Store {
    pub fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await {
            Ok(pool) => pool,
            Err(e) => panic!("Couldn't establish DB connection!"),
        };

        Store {
            connection: db_pool,
        }
    }
}
