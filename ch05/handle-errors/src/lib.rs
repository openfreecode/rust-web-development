use warp::{body::BodyDeserializeError, cors::CorsForbidden, reject::Reject, Rejection, Reply};

#[derive(Debug)]
pub enum Error {
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

pub async fn return_error(err: warp::Rejection) -> Result<impl Reply, Rejection> {
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
