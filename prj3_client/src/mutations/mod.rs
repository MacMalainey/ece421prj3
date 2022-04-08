pub mod auth;
pub mod match_records;

mod util;

#[derive(Debug, PartialEq, Clone)]
pub enum ServiceError {
    UnableToContactServer,
    InternalServerError
}

impl std::fmt::Display for ServiceError {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", match self {
            ServiceError::UnableToContactServer => "unable to contact server",
            ServiceError::InternalServerError => "internal server error"
        })
    }

}

impl std::error::Error for ServiceError {}

impl From<util::APIError> for ServiceError {
    
    fn from(err: util::APIError) -> Self {
        use util::APIError::*;

        match err {
            AuthenticationError => panic!(
                "ServiceError: Cannot construct from AuthenticationError as it assumes an unauthenticated API"
            ), // TODO: Add to error message reference for AuthenticatedServiceError when made
            InternalServerError => ServiceError::InternalServerError,
            TimeoutError => ServiceError::UnableToContactServer,
            RequestError(rerr) => panic!(
                "ServiceError: Cannot construct from RequestError.\nServices MUST internally handle RequestErrors themselves.\nError found:{:#?}", rerr
            ),
            UnknownStatus(status) if status >= 500 && status <= 599 => ServiceError::InternalServerError,
            UnknownStatus(status) => panic!(
                "ServiceError: Cannot construct from unidentified status {}", status
            ),
        }
    }
}
