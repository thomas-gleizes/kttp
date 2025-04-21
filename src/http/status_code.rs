#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StatusCode(u16);

impl StatusCode {
    pub const CONTINUE: StatusCode = StatusCode(100);
    pub const OK: StatusCode = StatusCode(200);
    pub const CREATED: StatusCode = StatusCode(201);
    pub const NO_CONTENT: StatusCode = StatusCode(204);
    pub const MOVED_PERMANENTLY: StatusCode = StatusCode(301);
    pub const FOUND: StatusCode = StatusCode(302);
    pub const BAD_REQUEST: StatusCode = StatusCode(400);
    pub const UNAUTHORIZED: StatusCode = StatusCode(401);
    pub const FORBIDDEN: StatusCode = StatusCode(403);
    pub const NOT_FOUND: StatusCode = StatusCode(404);
    pub const INTERNAL_SERVER_ERROR: StatusCode = StatusCode(500);

    pub fn new(code: u16) -> Option<StatusCode> {
        match code {
            100..=599 => Some(StatusCode(code)),
            _ => None,
        }
    }

    pub fn to_u16(&self) -> u16 {
        self.0
    }

    pub fn message(&self) -> &'static str {
        match self.0 {
            100 => "Continue",
            200 => "OK",
            201 => "Created",
            204 => "No Content",
            301 => "Moved Permanently",
            302 => "Found",
            400 => "Bad Request",
            401 => "Unauthorized",
            403 => "Forbidden",
            404 => "Not Found",
            500 => "Internal Server Error",
            // Codes moins courants
            code => standard_message(code),
        }
    }
}

// Fonction helper pour les messages standards
fn standard_message(code: u16) -> &'static str {
    match code {
        100..=199 => "Informational",
        200..=299 => "Success",
        300..=399 => "Redirection",
        400..=499 => "Client Error",
        500..=599 => "Server Error",
        _ => "Unknown Status Code",
    }
}
