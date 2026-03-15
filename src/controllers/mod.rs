use crate::error::AppError;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub mod ingredient;
pub mod recipe;

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status_code, body): (StatusCode, String) = match self {
            AppError::RecipeError(_err) => (StatusCode::BAD_REQUEST, "error".into()),
            AppError::IngredientError(_err) => (StatusCode::BAD_REQUEST, "error".into()),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".into(),
            ),
        };

        (status_code, body).into_response()
    }
}
