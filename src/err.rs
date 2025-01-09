use axum::{http::StatusCode, response::IntoResponse};

#[derive(Debug, thiserror::Error)]
pub enum MError {
    #[error("Message: {0}")]
    Message(String),
    #[error("IO Error: {0}")]
    IO(#[from] std::io::Error),
    #[error("Reqwest Error: {0}")]
    Web(#[from] axum::Error),
    #[error("Sqlx Error: {0}")]
    DB(#[from] sqlx::Error),
}

pub type MResult<T> = Result<T, MError>;

impl MError {
    pub fn msg<T>(msg: &str) -> MResult<T> {
        Err(MError::Message(msg.to_string()))
    }
}

pub trait IntoMResult<T> {
    fn into_mresult(self) -> MResult<T>;
}

impl<T, E> IntoMResult<T> for Result<T, E>
where
    E: Into<MError>
{
    fn into_mresult(self) -> MResult<T> {
        self.map_err(|e| e.into())
    }
}

impl IntoResponse for MError {
    fn into_response(self) -> axum::response::Response {
        log::error!("Server Error: {self}");
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}