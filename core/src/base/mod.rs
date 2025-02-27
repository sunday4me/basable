use std::{collections::HashMap, fmt::Display, sync::{Arc, Mutex}};

use axum::{body::Body, http::{Response, StatusCode}, response::IntoResponse};

use self::foundation::{BasableConnection, TableSummary};

pub(crate) mod auth;
pub(crate) mod config;
pub(crate) mod foundation;

pub(crate) type ConnectionStatus = HashMap<String, String>;
pub(crate) type TableSummaries = Vec<TableSummary>;
type SharedConnection = Arc<Mutex<dyn BasableConnection<Error = AppError>>>;

#[derive(Debug)]
pub(crate) struct AppError(pub StatusCode, pub String);
impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.0, self.1)
    }
}

/// Implements conversion of `mysql::Error` to AppError. At the moment, all variations
/// of `mysql::Error` resolves to `StatusCode::INTERNAL_SERVER_ERROR`.
impl From<mysql::Error> for AppError {
    fn from(value: mysql::Error) -> Self {
        Self(StatusCode::INTERNAL_SERVER_ERROR, value.to_string())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response<Body> {
        (self.0, self.1).into_response()
    }
}
