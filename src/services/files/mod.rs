use std::{io::Write, path::Path, sync::Arc};

use actix_multipart::{form::{tempfile::TempFile, MultipartForm}, Multipart};
use actix_web::web;
use futures_util::TryStreamExt;
use serde::Deserialize;
use uuid::Uuid;

pub struct FilesService;

pub enum FilesServiceErr {
    Internal,
    NoFilesToUpload
}

#[derive(Deserialize)]
pub struct FileName {
    file: String,
}

pub trait UploadPathProvider {
    fn upload_path(&self) -> String;
}

impl FilesService {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn save_file<T>(files: Vec<TempFile>, config: T) -> Result<FileName, FilesServiceErr> 
    where
        T: UploadPathProvider
    {
        let first_file = files.first().ok_or(FilesServiceErr::NoFilesToUpload)?;

        let filename = Arc::new(Uuid::new_v4().to_string());
        let directory = Arc::new(config.upload_path());


        Ok(FileName { file: filename.to_string() })
    }
}
