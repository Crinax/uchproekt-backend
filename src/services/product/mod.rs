use sea_orm::{DatabaseConnection, EntityTrait};

use entity::product::Entity as Product;
use serde::Serialize;

pub struct ProductService {
    db: DatabaseConnection,
}

#[derive(Copy, Clone, Debug)]
pub enum ProductServiceErr {
    Internal,
}

#[derive(Clone, Debug, Serialize)]
pub struct ProductSerializable {
    id: i32,
    name: String,
    price: rust_decimal::Decimal,
    article: String,
    description: String,
    photo: String,
}

impl ProductService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn all(&self) -> Result<Vec<ProductSerializable>, ProductServiceErr> {
        Product::find()
            .all(&self.db)
            .await
            .map(|models| {
                models
                    .iter()
                    .map(|model| ProductSerializable {
                        id: model.id,
                        name: model.name.clone(),
                        price: model.price,
                        article: model.article.clone(),
                        description: model.description.clone(),
                        photo: model.photo.clone(),
                    })
                    .collect::<Vec<ProductSerializable>>()
            })
            .map_err(|_| ProductServiceErr::Internal)
    }
}
