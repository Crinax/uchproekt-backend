use actix_multipart::form::MultipartForm;
use actix_web::{web::Data, HttpResponse, Responder};

use crate::{api::errors::ApiError, config::Config, services::files::{FilesService, FilesServiceErr}};

use super::dto::UploadForm;

pub(super) async fn create_file(
    MultipartForm(form): MultipartForm<UploadForm>,
    files_service: Data<FilesService>,
    config: Data<Config>
) -> impl Responder {
    let result = files_service.save_file(form.files, config.as_ref())
        .await
        .map_err(|err| match err {
            FilesServiceErr::Internal => ApiError::internal_error(),
            FilesServiceErr::NoFilesToUpload => ApiError::invalid_data(),
            FilesServiceErr::ForbiddenException => ApiError::invalid_data(),
            FilesServiceErr::MaxFileSizeExceed => ApiError::invalid_data(),
        })
        .map(|res| HttpResponse::Ok().json(res));

    if result.is_err() {
        return result.unwrap_err();
    }

    result.unwrap()
}
