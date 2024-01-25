use actix_web::{
    cookie::{
        time::{ext::NumericalDuration, OffsetDateTime},
        Cookie,
    },
    post,
    web::Data,
    HttpRequest, HttpResponse, Responder,
};

use crate::{
    api::{errors::ApiError, v1::auth::AuthDataResult, JsonMessage},
    cache::Cache,
    config::Config,
    services::auth::{AuthService, AuthServiceError},
};

#[post("refresh-tokens")]
pub(super) async fn refresh_tokens(
    req: HttpRequest,
    cache: Data<Cache>,
    config: Data<Config>,
    auth_service: Data<AuthService>,
) -> impl Responder {
    let refresh_token = req.cookie("refresh_token");
    let refresh_token_not_found = HttpResponse::Unauthorized().json(JsonMessage {
        message: "refresh_token_not_found",
    });
    let internal_error = ApiError::internal_error();

    if refresh_token.is_none() {
        return refresh_token_not_found;
    }

    let refresh_token = refresh_token.unwrap();
    let refresh_token = refresh_token.value();
    let access_token = cache.get_pair(refresh_token);

    if access_token.is_err() {
        return internal_error;
    }

    let access_token = access_token.unwrap();

    if access_token.is_none() {
        return refresh_token_not_found;
    }

    let access_token = access_token.unwrap();

    if refresh_token.is_empty() {
        return refresh_token_not_found;
    }

    let user_data = AuthService::decrypt_token(&access_token, config.as_ref());

    if let Err(err) = user_data {
        match err {
            AuthServiceError::InvalidToken => {
                return HttpResponse::BadRequest().json(JsonMessage {
                    message: "invalid_token",
                })
            }
            _ => return internal_error,
        }
    }

    let user_data = user_data.unwrap();
    let service_result = auth_service
        .refresh_tokens(&user_data, config.as_ref())
        .await;

    if let Err(err) = service_result {
        match err {
            AuthServiceError::UserNotFound => {
                return HttpResponse::NotFound().json(JsonMessage {
                    message: "user_not_found",
                })
            }
            _ => return internal_error,
        }
    }

    let tokens = service_result.unwrap();

    let _ = cache.remove(refresh_token);
    let _ = cache.add_pair(&tokens.1, &access_token, tokens.3);

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
            expires: tokens.2 * 1000,
        })
}
