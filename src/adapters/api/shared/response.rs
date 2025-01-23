use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub code: u16,
    pub message: String,
    pub result: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn new(code: u16, message: String, result: Option<T>) -> Self {
        Self {
            code,
            message,
            result,
        }
    }
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    pub fn success(result: T) -> Self {
        ApiResponse::new(200, "Success".to_string(), Some(result))
    }

    pub fn error(result: T) -> Self {
        ApiResponse::new(500, "Server Error".to_string(), Some(result))
    }
}
