use std::{fs, io::Read, path::Path};

use actix_multipart::form::tempfile::TempFile;
use sea_orm::{DatabaseConnection, EntityTrait, Set};
use serde::Serialize;
use uuid::Uuid;

use entity::file::{self, Entity as File};

pub struct FilesService {
    db: DatabaseConnection
}

pub enum FilesServiceErr {
    Internal,
    NoFilesToUpload,
    NotFound,
    ForbiddenFileType,
    MaxFileSizeExceed
}

#[derive(Serialize)]
pub struct FileName {
    file: String,
}

impl From<file::Model> for FileName {
    fn from(value: file::Model) -> Self {
        Self {
            file: value.id.to_string()
        }
    }
}

pub trait UploadPathProvider {
    fn upload_path(&self) -> &str;
}

impl FilesService {
    const MAX_FILE_SIZE: usize = 5_242_880;
    const PNG_FILE_SIGNATURE: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
    const JPEG_FILE_SIGNATURE: [u8; 4] = [0xFF, 0xD8, 0xFF, 0xEE];
    const JPG_FILE_SIGNATURE: [u8; 4] = [0xFF, 0xD8, 0xFF, 0xE0];

    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    fn has_valid_signature(&self, file: &mut TempFile) -> Result<bool, FilesServiceErr> {
        let mut buf: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
        
        file.file.read(&mut buf)
            .map_err(|err| {
                log::error!("{:?}", err);
                FilesServiceErr::Internal
            })?;

        log::info!("{:?}", buf);

        Ok(self.is_png(&buf) || self.is_jpeg(&buf[..4]))
    }

    fn is_png(&self, buf: &[u8; 8]) -> bool {
        buf == &FilesService::PNG_FILE_SIGNATURE
    }

    fn is_jpeg(&self, buf: &[u8]) -> bool {
        buf == &FilesService::JPEG_FILE_SIGNATURE || buf == &FilesService::JPG_FILE_SIGNATURE
    }

    pub async fn get_file<T>(&self, uid: Uuid, config: &T) -> Result<String, FilesServiceErr>
        where
            T: UploadPathProvider,
    {
        let db_file = File::find_by_id(uid).one(&self.db).await
            .map_err(|_| FilesServiceErr::Internal)?;

        if db_file.is_none() {
            return Err(FilesServiceErr::NotFound)
        }

        let db_file = db_file.unwrap();

        let directory = config.upload_path();
        let path = Path::new(&directory).join(db_file.filename);

        path.into_os_string().into_string().map_err(|_| FilesServiceErr::Internal)
    }

    pub async fn save_file<T>(
        &self,
        files: Vec<TempFile>,
        config: &T
    ) -> Result<FileName, FilesServiceErr> 
    where
        T: UploadPathProvider
    {

        let mut f = files.into_iter().nth(0).ok_or(FilesServiceErr::NoFilesToUpload)?;

        if f.size > FilesService::MAX_FILE_SIZE {
            return Err(FilesServiceErr::MaxFileSizeExceed)
        } 

        log::warn!("{:?}", f.content_type);

        if !self.has_valid_signature(&mut f)? {
            return Err(FilesServiceErr::ForbiddenFileType)
        }

        let full_filename = f.file_name.ok_or(FilesServiceErr::ForbiddenFileType)?;
        let ext = full_filename.split('.').last().ok_or(FilesServiceErr::ForbiddenFileType)?;

        let uuid = Uuid::new_v4();
        let filename = format!("{uuid}.{ext}");
        let directory = config.upload_path();
        let path = Path::new(&directory).join(&filename);

        fs::copy(f.file.path(), path)
            .map_err(|err| {
                log::error!("{:?}", err);
                FilesServiceErr::Internal
            })?;

        let file_data = file::ActiveModel {
            id: Set(uuid),
            filename: Set(filename),
            ..Default::default()
        };

        File::insert(file_data)
            .exec(&self.db)
            .await
            .map(|model| FileName { file: model.last_insert_id.to_string() })
            .map_err(|_| FilesServiceErr::Internal)
    }
}
