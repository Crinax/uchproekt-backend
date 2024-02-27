use actix_web::{web::{Data, Json}, HttpResponse, Responder};
use validator::Validate;

use crate::{api::{errors::ApiError, v1::categories::dto::DeleteCategoriesDto}, services::category::CategoryService};

pub(super) async fn delete_categories(
    data: Json<DeleteCategoriesDto>,
    category_service: Data<CategoryService>
) -> impl Responder {
    if data.validate().is_err() {
        return ApiError::invalid_data();
    }

    let deletion_result = category_service.delete(&data.0.categories)
        .await
        .map(|data| HttpResponse::Ok().json(data));

    if deletion_result.is_err() {
        return ApiError::internal_error();
    }

    deletion_result.unwrap()
}
