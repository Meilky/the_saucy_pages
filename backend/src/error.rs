#[derive(Debug)]
pub enum IngredientError {
    NameTooShort,
    NameTooLong,
    NameAlreadyUsed,
    DescriptionTooShort,
    DescriptionTooLong,
}

#[derive(Debug)]
pub enum RecipeError {
    NameTooShort,
    NameTooLong,
    NameAlreadyUsed,
    DescriptionTooShort,
    DescriptionTooLong,
}

#[derive(Debug)]
pub enum AppError {
    InternalServerError,
    RecipeError(RecipeError),
    IngredientError(IngredientError),
}

impl From<RecipeError> for AppError {
    fn from(value: RecipeError) -> Self {
        AppError::RecipeError(value)
    }
}

impl From<IngredientError> for AppError {
    fn from(value: IngredientError) -> Self {
        AppError::IngredientError(value)
    }
}

impl From<sqlx::Error> for AppError {
    fn from(_value: sqlx::Error) -> Self {
        AppError::InternalServerError
    }
}
