use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use warp::{
    body::BodyDeserializeError, cors::CorsForbidden, http::Method, reject::Reject, Filter,
    Rejection, Reply,
};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Clone)]
struct QuestionId(String);

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Clone)]
struct AnswerId(String);

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Answer {
    id: AnswerId,
    content: String,
    question_id: QuestionId,
}

#[derive(Clone)]
struct Store {
    questions: Arc<RwLock<HashMap<QuestionId, Question>>>,
    answers: Arc<RwLock<HashMap<AnswerId, Answer>>>,
}

impl Store {
    fn new() -> Self {
        Store {
            questions: Arc::new(RwLock::new(Store::init())),
            answers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn init() -> HashMap<QuestionId, Question> {
        let file = include_str!("../questions.json");
        serde_json::from_str(file).expect("can't read questions.json")
    }
}

#[derive(Debug)]
struct Pagination {
    start: usize,
    end: usize,
}

#[derive(Debug)]
enum Error {
    ParseError(std::num::ParseIntError),
    MissingParameters,
    QuestionNotFound,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ParseError(e) => write!(f, "Cannot parse parameter: {}", e),
            Error::MissingParameters => write!(f, "Missing parameters"),
            Error::QuestionNotFound => write!(f, "Question not found"),
        }
    }
}

impl Reject for Error {}

fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    if params.contains_key("start") && params.contains_key("end") {
        return Ok(Pagination {
            start: params
                .get("start")
                .unwrap()
                .parse::<usize>()
                .map_err(Error::ParseError)?,
            end: params
                .get("end")
                .unwrap()
                .parse::<usize>()
                .map_err(Error::ParseError)?,
        });
    }

    return Err(Error::MissingParameters);
}

async fn get_questions(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    if !params.is_empty() {
        let pagination = extract_pagination(params)?;
        let res: Vec<Question> = store.questions.read().await.values().cloned().collect();
        let res = &res[pagination.start..pagination.end];
        Ok(warp::reply::json(&res))
    } else {
        let res: Vec<Question> = store.questions.read().await.values().cloned().collect();
        Ok(warp::reply::json(&res))
    }
}

async fn get_question(id: String, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    match store.questions.read().await.get(&QuestionId(id)) {
        Some(q) => Ok(warp::reply::json(q)),
        None => Err(warp::reject::custom(Error::QuestionNotFound)),
    }
}

async fn add_question(
    question: Question,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    store
        .questions
        .write()
        .await
        .insert(question.id.clone(), question);
    Ok(warp::reply::with_status(
        "Questuin added",
        warp::http::StatusCode::CREATED,
    ))
}

async fn update_question(
    id: String,
    question: Question,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.questions.write().await.get_mut(&QuestionId(id)) {
        Some(q) => {
            *q = question;
        }
        None => return Err(warp::reject::custom(Error::QuestionNotFound)),
    }

    Ok(warp::reply::with_status(
        "Questuin updated",
        warp::http::StatusCode::OK,
    ))
}

async fn delete_question(id: String, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    match store.questions.write().await.remove(&QuestionId(id)) {
        Some(_) => Ok(warp::reply::with_status(
            "Questuin deleted",
            warp::http::StatusCode::OK,
        )),
        None => Err(warp::reject::custom(Error::QuestionNotFound)),
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Answerx {
    content: String,
}

async fn add_answer(
    question_id: String,
    answer: Answerx,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    store
        .questions
        .read()
        .await
        .get(&QuestionId(question_id.to_string()))
        .ok_or(Error::QuestionNotFound)?;

    let answer = Answer {
        id: AnswerId(uuid::Uuid::new_v4().to_string()),
        content: answer.content,
        question_id: QuestionId(question_id.to_string()),
    };
    store
        .answers
        .write()
        .await
        .insert(answer.id.clone(), answer);
    Ok(warp::reply::with_status(
        "Answer added",
        warp::http::StatusCode::CREATED,
    ))
}

async fn return_error(err: warp::Rejection) -> Result<impl Reply, Rejection> {
    println!("err: {:?}", err);
    let code;
    let message;

    if let Some(e) = err.find::<Error>() {
        match e {
            Error::ParseError(_) => {
                code = warp::http::StatusCode::BAD_REQUEST;
                message = e.to_string();
            }
            Error::MissingParameters => {
                code = warp::http::StatusCode::BAD_REQUEST;
                message = e.to_string();
            }
            Error::QuestionNotFound => {
                code = warp::http::StatusCode::NOT_FOUND;
                message = e.to_string();
            }
        }
    } else if let Some(e) = err.find::<CorsForbidden>() {
        code = warp::http::StatusCode::FORBIDDEN;
        message = e.to_string();
    } else if let Some(e) = err.find::<BodyDeserializeError>() {
        code = warp::http::StatusCode::UNPROCESSABLE_ENTITY;
        message = e.to_string();
    } else {
        return Err(err);
    }

    Ok(warp::reply::with_status(message, code))
}

#[tokio::main]
async fn main() {
    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::POST, Method::GET, Method::DELETE]);

    let get_questions = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query::<HashMap<String, String>>())
        .and(store_filter.clone())
        .and_then(get_questions);

    let get_question = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(get_question);

    let add_question = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and(store_filter.clone())
        .and_then(add_question);

    let update_question = warp::put()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(warp::body::json())
        .and(store_filter.clone())
        .and_then(update_question);

    let delete_question = warp::delete()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(delete_question);

    let add_answer = warp::post()
        .and(warp::path!("questions" / String / "answers"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and(store_filter.clone())
        .and_then(add_answer);

    let routes = get_questions
        .or(get_question)
        .or(add_question)
        .or(update_question)
        .or(delete_question)
        .or(add_answer)
        .with(cors)
        .recover(return_error);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
