use std::path::Path;

use actix_multipart::form::tempfile::TempFile;
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

        let filename = Uuid::new_v4().to_string();
        let directory = config.upload_path();
        let path = Path::new(&directory).join(&filename);

        let f = files.into_iter().nth(0).ok_or(FilesServiceErr::NoFilesToUpload)?;
        let _file = f.file.persist(path).map_err(|_| FilesServiceErr::Internal)?;


        Ok(FileName { file: filename.to_string() })
    }
}
