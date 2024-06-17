use serde::Serialize;


#[derive(Serialize)]
pub struct ResultResponse {
    pub status: String,
}


impl ResultResponse {
    pub fn new(status: &str) -> ResultResponse {
        ResultResponse { status: String::from(status) }
    }
}
