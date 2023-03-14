use std::io::{Error, ErrorKind};
use std::str::FromStr;

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
    fn new(
        id: QuestionId,
        title: String,
        content: String,
        tags: Option<Vec<String>>,
    ) -> Self {
        id,
        title,
        content,
        tags,
    }

    fn fmt(&self, f:&mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}, title: {}, content: {}, tags: {:?",
            self.id, self.title, self.content, self.tags
        )
    }
}

impl std::fmt::Display for Question {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "id: {:?}", self.tags)
    }
}

assert_eq!("(1.987, 2.983)",
    format!(
        "{}",
        Position {
            longitude: 1.987, latitude: 2.983,
        }
    )
);

impl FromStr for QuestionId {
    type Err = std::io::Error;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(QuestionId(id.to_string())),
            true => Err(
                Error::new(ErrorKind::InvalidInput, "No id provided")
            ),
        }
    }
}

fn main() {
    let hello = warp::get()
        .map(|| format!("Hello, World!"));

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
