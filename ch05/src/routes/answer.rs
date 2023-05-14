use crate::{
    store::Store,
    types::{Answer, AnswerId, QuestionId},
};
use handle_errors::Error;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Answerx {
    pub content: String,
}

pub async fn add_answer(
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
