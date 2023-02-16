use actix_web::{http::StatusCode, HttpResponse, ResponseError};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Internal Server Error")]
    Internal,
    #[error("{0} Not Found")]
    NotFound(String),
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    DeserializeError(#[from] serde_json::Error),
    #[error("{0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("{0}")]
    Unauthorized(String),
    #[error("{0}")]
    Forbidden(String),
    #[error("{0}")]
    SessionGetError(#[from] actix_session::SessionGetError),
    #[error("{0}")]
    SessionInsertError(#[from] actix_session::SessionInsertError),
    #[error("{message}")]
    Custom { code: StatusCode, message: String },
    #[error("This error should not normally occur. If it occurs repeatedly, please contact our developers.")]
    Unimplemented,
    #[error("Invalid Response Body: {0}")]
    InvalidResponseBody(reqwest::Error),
    #[error("Invalid Url: {0}")]
    InvalidUrl(url::ParseError),
    #[error("{{0.message}}")]
    ApiError(ErrorResponse),
    #[error("Server Internal Error")]
    IoError(std::io::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    code: u16,
    message: String,
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            Error::NotFound(_) => StatusCode::NOT_FOUND,
            Error::BadRequest(_) => StatusCode::BAD_REQUEST,
            Error::Forbidden(_) => StatusCode::FORBIDDEN,
            Error::DeserializeError(_) => StatusCode::BAD_REQUEST,
            Error::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Error::SessionGetError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::SessionInsertError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Custom { code, .. } => *code,
            Error::Unimplemented => StatusCode::SERVICE_UNAVAILABLE,
            Error::InvalidResponseBody(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::InvalidUrl(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::ApiError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::IoError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_response = ErrorResponse {
            code: status_code.as_u16(),
            message: self.to_string(),
        };
        HttpResponse::build(status_code).json(error_response)
    }
}

pub fn create_error(code: StatusCode, message: &str) -> Error {
    Error::Custom {
        code,
        message: message.to_string(),
    }
}

impl From<actix_web::Error> for Error {
    fn from(error: actix_web::Error) -> Self {
        Error::Custom {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: error.to_string(),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        error!("!! IO Error occured!: {:?}", error);
        Error::IoError(error)
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::Custom {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: error.to_string(),
        }
    }
}

impl From<stripe::StripeError> for Error {
    fn from(error: stripe::StripeError) -> Self {
        error!("Stripe Error: {:?}", error);
        Error::Custom {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: error.to_string(),
        }
    }
}

impl From<anyhow::Error> for Error {
    fn from(error: anyhow::Error) -> Self {
        Error::Custom {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: error.to_string(),
        }
    }
}

impl Error {
    pub async fn from_response(response: reqwest::Response) -> Self {
        let body = response.text().await;
        let body = match body {
            Ok(body) => body,
            Err(error) => {
                error!("Error while parsing response: {:?}", error);
                return Error::InvalidResponseBody(error);
            }
        };
        let error = match serde_json::from_str::<ErrorResponse>(&body) {
            Ok(error) => error,
            Err(error) => {
                error!("Error while parsing response: {:?}", error);
                return Error::DeserializeError(error);
            }
        };
        Error::ApiError(error)
    }

    pub fn internal<S>(message: S) -> Self
    where
        S: Into<String>,
    {
        error!("Internal Server Error: {}", message.into());
        Error::Internal
    }

    pub fn forbidden<S>(message: S) -> Self
    where
        S: Into<String>,
    {
        Error::Forbidden(message.into())
    }

    pub fn unauthorized<S>(message: S) -> Self
    where
        S: Into<String>,
    {
        Error::Unauthorized(message.into())
    }

    pub fn unimplemented() -> Self {
        Error::Unimplemented
    }

    pub fn not_found<S>(resource: S) -> Self
    where
        S: Into<String>,
    {
        Error::NotFound(resource.into())
    }

    pub fn bad_request<S>(message: S) -> Self
    where
        S: Into<String>,
    {
        Error::BadRequest(message.into())
    }
}
