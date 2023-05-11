use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use warp::{http::Method, Filter, Rejection, Reply};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Clone)]
struct QuestionId(String);

async fn get_questions(store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    let res: Vec<Question> = store.questions.values().cloned().collect();
    Ok(warp::reply::json(&res))
}

async fn return_error(err: warp::Rejection) -> Result<impl Reply, Rejection> {
    println!("err: {:?}", err);
    let code;
    let message;

    if err.is_not_found() {
        code = warp::http::StatusCode::NOT_FOUND;
        message = "Not Found";
    } else {
        code = warp::http::StatusCode::INTERNAL_SERVER_ERROR;
        message = "Internal Server Error";
    }

    Ok(warp::reply::with_status(message, code))
}

#[derive(Clone)]
struct Store {
    questions: HashMap<QuestionId, Question>,
}

impl Store {
    fn new() -> Self {
        Store {
            questions: Self::init(),
        }
    }

    fn init() -> HashMap<QuestionId, Question> {
        let file = include_str!("../questions.json");
        serde_json::from_str(file).expect("can't read questions.json")
    }
}

#[tokio::main]
async fn main() {
    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::POST, Method::GET, Method::DELETE]);

    let get_items = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(store_filter)
        .and_then(get_questions)
        .recover(return_error);

    let routes = get_items.with(cors);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
