use std::{fs, path::Path};

use actix_multipart::form::tempfile::TempFile;
use serde::Serialize;
use uuid::Uuid;

pub struct FilesService;

pub enum FilesServiceErr {
    Internal,
    NoFilesToUpload,
    ForbiddenException,
    MaxFileSizeExceed
}

#[derive(Serialize)]
pub struct FileName {
    file: String,
}

pub trait UploadPathProvider {
    fn upload_path(&self) -> &str;
}

impl FilesService {
    const MAX_FILE_SIZE: usize = 5_242_880;

    pub fn new() -> Self {
        Self {}
    }

    pub async fn save_file<T>(
        &self,
        files: Vec<TempFile>,
        config: &T
    ) -> Result<FileName, FilesServiceErr> 
    where
        T: UploadPathProvider
    {

        let f = files.into_iter().nth(0).ok_or(FilesServiceErr::NoFilesToUpload)?;

        if f.size > FilesService::MAX_FILE_SIZE {
            return Err(FilesServiceErr::MaxFileSizeExceed)
        } 

        let full_filename = f.file_name.ok_or(FilesServiceErr::ForbiddenException)?;
        let ext = full_filename.split(".").last().ok_or(FilesServiceErr::ForbiddenException)?;

        let filename = Uuid::new_v4().to_string() + "." + ext;
        let directory = config.upload_path();
        let path = Path::new(&directory).join(&filename);

        log::info!("{:?}", &path);
        log::info!("{:?}", f.file.path());

        fs::copy(f.file.path(), path)
            .map_err(|err| {
                log::error!("{:?}", err);
                FilesServiceErr::Internal
            })?;


        Ok(FileName { file: filename.to_string() })
    }
}
