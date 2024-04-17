use actix_web::{
    get, http::header::{ContentDisposition, DispositionType}, web::{self, Data}, HttpRequest, Responder
};
use actix_files::NamedFile;
use uuid::Uuid;

use crate::{api::errors::ApiError, config::Config, services::files::{FilesService, FilesServiceErr}};

#[get("/{filename:.*}")]
pub(super) async fn get_file(
    req: HttpRequest,
    filename: web::Path<Uuid>, 
    file_service: Data<FilesService>,
    config: Data<Config>,
) -> impl Responder {
    let uid = filename.into_inner();

    let path = file_service.get_file(uid, config.as_ref())
        .await
        .map_err(|err| match err {
            FilesServiceErr::NotFound => ApiError::not_found(),
            _ => ApiError::internal_error()
        });

    if path.is_err() {
        return path.err().unwrap();
    }

    let path = path.unwrap();

    let file = NamedFile::open(path).map_err(|_| ApiError::internal_error());

    if file.is_err() {
        return file.err().unwrap();
    }

    let file = file.unwrap();

    file
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: vec![],
        })
        .into_response(&req)
}
