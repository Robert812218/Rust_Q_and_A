use std::collections::HashMap;

use tracing::{event, instrument, Level};
use warp::http::StatusCode;

use crate::store::Store;
use crate::types::pagination::{extract_pagination, Pagination};
use crate::types::question::{NewQuestion, Question};

#[instrument]
pub async fn get_questions(
    &self,
    limit: Option<i32>,
    offset: i32,
) -> Result<impl warp::Reply, warp::Rejection> {
    match sqlx::query("SELECT * from questions LIMIT $1 OFFSET $2")
        .bind(limit)
        .bind(offset)
        .map(|row: PgRow| {
            id: QuestionId(row.get("id")),
            title: row.get("tags"),
            })
        .fetch_all(&self.connection)
        .await {
            Ok(questions) => Ok(questions),
            Err(e) => Err(e),
        }
}

pub async fn update_question(
    id: i32,
    store: Store,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.update_question(question, id).await {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn delete_question(
    id: i32,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.delete_question(id).await {
        Ok(_) => Ok(warp::reply::with_status(
            format!("Question {} deleted", id),
            StatusCode::OK,
        )),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn add_question(
    store: Store,
    new_question: NewQuestion,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.add_question(new_question).await {
        Ok(_) => {
            Ok(warp::reply::with_status("Question added", StatusCode::OK))
        }
        Err(e) => Err(warp::reject::custom(e)),
    }
}
