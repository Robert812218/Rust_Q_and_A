use std::str::FromStr;
use std::io::{Error, ErrorKind};
use warp::{Filter, reject::Reject, Rejection, Reply, http::StatusCode};
use serde::Serialize;

#[derive(Debug)]
struct Question {
    id: QuestionId(String),
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
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

struct InvalidId;
impl Reject for InvalidId {}

async function get_questions() -> Result<impl warp::Reply, warp::Rejection> {
    let question = Question::new(
            QuestionId::from_str("1").expect("No id provided"),
            "First Question".to_string(),
            "Content of Question".to_string(),
            Some(vec!("faq".to_string())),            
        );

        match question.id.0.parse::<i32>() {
            Err(_) => {
                Err(warp::reject::custom(InvalidId))
            },
            Ok(_) => {
                Ok(warp::reply::json(
                    &question
                ))
            }
        }
}

async function return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(_InvalidId) = r.find() {
        Ok(warp::reply::with_status(
            "No valid ID presented",
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else {
        "Route not found",
        StatusCode::NOT_FOUND,
    }
}

#[tokio::main]
async fn main() {
    let get_items = warp::get()
        .and(warp::path("questions")))
        .and(warp::path::end())
        .and_then(get_questions)
        .recover(error);

    let routes = get_items;

    warp::routes(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

