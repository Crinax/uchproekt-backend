use actix_web::{
    cookie::{
        time::{ext::NumericalDuration, OffsetDateTime},
        Cookie,
    },
    post,
    web::Data,
    HttpRequest, HttpResponse, Responder,
};

use crate::{api::JsonMessage, cache::Cache};

#[post("logout")]
pub(super) async fn logout(req: HttpRequest, cache: Data<Cache>) -> impl Responder {
    let refresh_token = req.cookie("refresh_token");

    if refresh_token.is_none() {
        return HttpResponse::Ok().json(JsonMessage {
            message: "already_removed",
        });
    }

    let refresh_token = refresh_token.unwrap().to_string();
    let _ = cache.remove(&refresh_token);
    let expires_time = OffsetDateTime::from_unix_timestamp(0);

    HttpResponse::Ok()
        .cookie(
            Cookie::build("refresh_token", refresh_token)
                .secure(true)
                .http_only(true)
                .path("/api/v1/auth")
                .expires(expires_time.unwrap_or(OffsetDateTime::now_utc() - 30.days()))
                .finish(),
        )
        .json(JsonMessage { message: "ok" })
}
