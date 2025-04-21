use crate::http::status_code::StatusCode;
use std::collections::HashMap;

struct Response {
    status: StatusCode,
    headers: HashMap<String, String>,
    body: String,
}
