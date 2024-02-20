use actix_web::{get, web::Data, HttpResponse, Responder};

use crate::{api::errors::ApiError, services::category::CategoryService};

#[get("")]
pub(super) async fn get_categories(category_service: Data<CategoryService>) -> impl Responder {
    let categories = category_service.all().await;

    if categories.is_err() {
        return ApiError::internal_error();
    }

    categories.map(|data| HttpResponse::Ok().json(data)).unwrap()
}

#[get("/tree")]
pub(super) async fn get_tree_categories(category_service: Data<CategoryService>) -> impl Responder {
    let categories = category_service.all_tree().await;

    if categories.is_err() {
        return ApiError::internal_error();
    }

    categories.map(|data| HttpResponse::Ok().json(data)).unwrap()
}
