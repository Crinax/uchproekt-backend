use actix_web::{web::{Data, Json, Path}, HttpResponse, Responder};
use validator::Validate;

use crate::{api::errors::ApiError, services::category::{CategoryService, CategoriesServiceErr}};

use super::dto::UpdateCategoryDto;

pub(super) async fn patch_category(
    category_id: Path<u32>,
    dto: Json<UpdateCategoryDto>,
    category_service: Data<CategoryService>
) -> impl Responder {
    if dto.validate().is_err() {
        return ApiError::invalid_data();
    }

    let create_result = category_service.update(category_id.to_owned(), dto.name.as_deref(), dto.parent_id).await;

    if create_result.is_err() {
        return match create_result.err().unwrap() {
            CategoriesServiceErr::NotFound => ApiError::not_found(),
            CategoriesServiceErr::InvalidParentId => ApiError::invalid_data(),
            _ => ApiError::internal_error()
        }
    }

    create_result
        .map(|res| HttpResponse::Ok().json(res))
        .unwrap()
}
