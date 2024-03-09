use actix_multipart::form::{
    tempfile::TempFile,
    MultipartForm,
};
use validator::Validate;

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    #[multipart(rename = "file")]
    files: Vec<TempFile>,
}
