use actix_web::{
    cookie::{
        time::{ext::NumericalDuration, OffsetDateTime},
        Cookie,
    },
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use validator::Validate;

use crate::{
    api::{
        errors::ApiError,
        v1::auth::{dto::AuthorizationDto, AuthDataResult},
    },
    cache::Cache,
    config::Config,
    services::auth::{AuthService, AuthServiceError},
};

#[post("")]
pub(super) async fn authorize(
    json: Json<AuthorizationDto>,
    cache: Data<Cache>,
    auth_service: Data<AuthService>,
    config: Data<Config>,
) -> impl Responder {
    if json.validate().is_err() {
        return ApiError::invalid_data();
    }

    let db_result = auth_service
        .authorize_user(&json.0.username, &json.0.password, config.as_ref())
        .await;

    if let Err(db_err) = db_result {
        match db_err {
            AuthServiceError::UserNotFound => return ApiError::invalid_data(),
            AuthServiceError::InvalidPassword => return ApiError::invalid_data(),
            _ => return ApiError::internal_error(),
        }
    }

    let tokens = db_result.unwrap();
    let _ = cache.add_pair(&tokens.1, &tokens.0, tokens.3);
    let expires_time = OffsetDateTime::from_unix_timestamp(tokens.3 as i64 * 1000);

    HttpResponse::Ok()
        .cookie(
            Cookie::build("refresh_token", tokens.1)
                .secure(true)
                .http_only(true)
                .path("/api/v1/auth")
                .expires(expires_time.unwrap_or(OffsetDateTime::now_utc() + 30.days() * 1000))
                .finish(),
        )
        .json(AuthDataResult {
            access_token: tokens.0,
            expires: tokens.2,
        })
}
