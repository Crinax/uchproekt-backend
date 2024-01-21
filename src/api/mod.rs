pub(super) mod errors;

use serde::Serialize;

#[derive(Serialize)]
pub struct JsonMessage<'a> {
    pub message: &'a str,
}
