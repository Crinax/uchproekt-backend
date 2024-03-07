use std::{borrow::Borrow, io::Write, path::Path, sync::{Arc, Mutex}};

use actix_multipart::Multipart;
use actix_web::web;
use futures_util::TryStreamExt;
use serde::Deserialize;
use uuid::Uuid;

pub struct FilesService;

pub enum FilesServiceErr {
    Internal,
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

    pub async fn save_file<T>(mut payload: Multipart, config: T) -> Result<FileName, FilesServiceErr> 
    where
        T: UploadPathProvider
    {
        let filename = Arc::new(Uuid::new_v4().to_string());
        let directory = Arc::new(config.upload_path());

        while let Some(mut field) = payload.try_next().await.map_err(|_| FilesServiceErr::Internal)? {
            let clonned_filename = Arc::clone(&filename);
            let clonned_dir = Arc::clone(&directory);

            let mut file = web::block(
                move || {
                    let file_path = Path::new(&*clonned_dir).join(&*clonned_filename);

                    std::fs::File::create(file_path).map_err(|_| FilesServiceErr::Internal)
                }
            )
                .await
                .map_err(|_| FilesServiceErr::Internal)??;

            while let Some(chunk) = field.try_next().await.map_err(|_| FilesServiceErr::Internal)? {
                file = web::block(
                    move || file.write_all(&chunk)
                        .map(|_| file)
                        .map_err(|_| FilesServiceErr::Internal)
                )
                    .await
                    .map_err(|_| FilesServiceErr::Internal)??;
            }
        }

        Ok(FileName { file: filename.to_string() })
    }
}
