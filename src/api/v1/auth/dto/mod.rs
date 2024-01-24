use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Debug, Clone)]
pub struct AuthorizationDto {
    #[validate(length(min = 3, max = 255))]
    pub username: String,

    #[validate(length(min = 8, max = 32))]
    pub password: String,
}
