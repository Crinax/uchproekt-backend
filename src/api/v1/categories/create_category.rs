use actix_web::{web::{Data, Json}, HttpResponse, Responder};
use validator::Validate;

use crate::{api::errors::ApiError, services::category::{CategoryService, CategoriesServiceErr}};

use super::dto::CreateCategoryDto;

pub(super) async fn create_category(
    dto: Json<CreateCategoryDto>,
    category_service: Data<CategoryService>
) -> impl Responder {
    if dto.validate().is_err() {
        return ApiError::invalid_data();
    }

    let create_result = category_service.create(&dto.name, dto.parent_id).await;

    if create_result.is_err() {
        return match create_result.err().unwrap() {
            CategoriesServiceErr::AlreadyExists => ApiError::conflict(),
            CategoriesServiceErr::InvalidParentId => ApiError::invalid_data(),
            _ => ApiError::internal_error()
        }
    }

    create_result
        .map(|res| HttpResponse::Created().json(res))
        .unwrap()
}
