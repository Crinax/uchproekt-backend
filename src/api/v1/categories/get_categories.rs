use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse, Responder,
};

use crate::{
    api::errors::ApiError,
    services::category::{CategoriesServiceErr, CategoryService},
};

#[get("")]
pub(super) async fn get_categories(category_service: Data<CategoryService>) -> impl Responder {
    let categories = category_service.all().await;

    if categories.is_err() {
        return ApiError::internal_error();
    }

    categories
        .map(|data| HttpResponse::Ok().json(data))
        .unwrap()
}

#[get("/tree")]
pub(super) async fn get_tree_categories(category_service: Data<CategoryService>) -> impl Responder {
    let categories = category_service.all_tree().await;

    if categories.is_err() {
        return ApiError::internal_error();
    }

    categories
        .map(|data| HttpResponse::Ok().json(data))
        .unwrap()
}

#[get("{id}")]
pub(super) async fn get_category_with_products(
    path: Path<(u32,)>,
    category_service: Data<CategoryService>,
) -> impl Responder {
    let categories = category_service
        .category_with_products(path.0)
        .await
        .map_err(|err| match err {
            CategoriesServiceErr::NotFound => ApiError::not_found(),
            _ => ApiError::internal_error(),
        })
        .map(|value| HttpResponse::Ok().json(value));

    if let Err(err) = categories {
        return err;
    }

    categories.unwrap()
}
