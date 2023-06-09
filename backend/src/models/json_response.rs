use serde::Serialize;

#[derive(Serialize)]
pub struct JsonResponse {
    pub err : String
}

impl JsonResponse {
    pub fn ok() -> Self {
        Self { err: "".to_string() }
    }

    pub fn err(err : String) -> Self {
        Self { err }
    }
}
