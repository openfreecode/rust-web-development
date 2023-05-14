use crate::{
    store::Store,
    types::{pagination::extract_pagination, Question, QuestionId},
};
use handle_errors::Error;
use std::collections::HashMap;

pub async fn get_questions(
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

pub async fn get_question(id: String, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    match store.questions.read().await.get(&QuestionId(id)) {
        Some(q) => Ok(warp::reply::json(q)),
        None => Err(warp::reject::custom(Error::QuestionNotFound)),
    }
}

pub async fn add_question(
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

pub async fn update_question(
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

pub async fn delete_question(
    id: String,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.questions.write().await.remove(&QuestionId(id)) {
        Some(_) => Ok(warp::reply::with_status(
            "Questuin deleted",
            warp::http::StatusCode::OK,
        )),
        None => Err(warp::reject::custom(Error::QuestionNotFound)),
    }
}
