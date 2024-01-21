use sea_orm::{DatabaseConnection, DbErr, EntityTrait};

use entity::product::Entity as Product;
use serde::Serialize;

pub struct ProductService {
    db: DatabaseConnection,
}

#[derive(Copy, Clone, Debug)]
pub enum ProductServiceErr {
    Internal
}

#[derive(Clone, Debug, Serialize)]
pub struct ProductSerializable {
    id: i32,
    name: String,
    price: rust_decimal::Decimal,
    article: String,
    description: String,
}

impl ProductService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn all(&self) -> Result<Vec<ProductSerializable>, ProductServiceErr> {
        Product::find().all(&self.db).await
            .map(|models|
                models.iter().map(|model|
                    ProductSerializable {
                        id: model.id,
                        name: model.name.clone(),
                        price: model.price,
                        article: model.article.clone(),
                        description: model.description.clone()
                    }
                )
                .collect::<Vec<ProductSerializable>>()
            )
            .map_err(|err| match err {
                _ => ProductServiceErr::Internal
            })
    }
}
