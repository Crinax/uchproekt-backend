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
            FilesServiceErr::NotFound => ApiError::internal_error(),
            FilesServiceErr::Internal => ApiError::internal_error(),
            e => {
                log::error!("{:?}", e); 
                ApiError::invalid_data()
            },
        })
        .map(|res| HttpResponse::Ok().json(res));

    if let Err(err) = result {
        return err;
    }

    result.unwrap()
}
