use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponseError {
    err : String
}

pub enum ApiError {
    RegistrationFailed,
    AuthFailed,
    NotLoggedIn,
    QueryDiets,
    QueryDietNutrition,
    AccessDenied,
    QueryNutrients,
    QueryMeals,
    CreateDiet
}

impl ToString for ApiError {
    fn to_string(&self) -> String {
        match self {
            ApiError::RegistrationFailed => "Failed to register user (try again)".to_string(),
            ApiError::AuthFailed => "User authentication failed (check your credentials)".to_string(),
            ApiError::NotLoggedIn => "User is not logged in (missing session_id)".to_string(),
            ApiError::QueryDiets => "Failed to query user diets (try refreshing the page)".to_string(),
            ApiError::QueryDietNutrition => "Failed to query diet nutrition (try refreshing the page)".to_string(),
            ApiError::AccessDenied => "Access denied (user cannot access the requested resource)".to_string(),
            ApiError::QueryNutrients => "Failed to query nutrients (try refreshing the page)".to_string(),
            ApiError::QueryMeals => "Failed to query meals (try refreshing the page)".to_string(),
            ApiError::CreateDiet => "Failed to create diet (try again)".to_string()
        }
    }
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum ApiResponse<T> {
    Success(T),
    Failure(ApiResponseError)
}

impl<T> ApiResponse<T> {
    pub fn ok(data : T) -> Self {
        Self::Success(data)
    }

    pub fn err(error : ApiError) -> Self {
        Self::Failure(ApiResponseError { err: error.to_string() })
    }
}

