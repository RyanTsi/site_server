use std::result;

use axum::{http::StatusCode, response::IntoResponse};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    DbError(sqlx::Error),
    UnexpectedDataInModel,
    RowNotFound,
    UserNotFound,
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Error::DbError(e)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Error::DbError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
            }
            Error::UnexpectedDataInModel => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected data error").into_response()
            }
            Error::RowNotFound => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Data Not Found in database").into_response()
            },
            Error::UserNotFound => {
                (StatusCode::INTERNAL_SERVER_ERROR, "User Not Found in database").into_response()
            }
        }
    }
}