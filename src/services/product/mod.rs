use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use entity::product::{self, Entity as Product};
use serde::Serialize;

pub struct ProductService {
    db: DatabaseConnection,
}

#[derive(Copy, Clone, Debug)]
pub enum ProductServiceErr {
    Internal,
    NotFound
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

#[derive(Clone, Debug, Serialize)]
pub struct ProductIdx {
    idx: Vec<u32>
}

impl From<&[u32]> for ProductIdx {
    fn from(value: &[u32]) -> Self {
        ProductIdx {
            idx: value.into_iter().map(|v| *v).collect()
        }
    }
}

impl From<Vec<product::Model>> for ProductIdx {
    fn from(value: Vec<product::Model>) -> Self {
        ProductIdx {
            idx: value.into_iter().map(|v| v.id as u32).collect()
        }
    }
}

impl From<product::Model> for ProductSerializable {
    fn from(model: product::Model) -> Self {
        ProductSerializable {
            id: model.id,
            name: model.name,
            price: model.price,
            article: model.article,
            description: model.description,
            photo: model.photo,
        }
    }
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
                    .into_iter()
                    .map(Into::into)
                    .collect::<Vec<ProductSerializable>>()
            })
            .map_err(|_| ProductServiceErr::Internal)
    }

    pub async fn get(&self, id: u32) -> Result<ProductSerializable, ProductServiceErr> {
        Product::find_by_id(id as i32)
            .one(&self.db)
            .await
            .map_err(|_| ProductServiceErr::Internal)?
            .ok_or(ProductServiceErr::NotFound)
            .map(Into::into)
    }

    pub async fn delete(&self, idx: &[u32]) -> Result<ProductIdx, ProductServiceErr> {
        let values = idx.iter().map(|e| Into::<sea_orm::Value>::into(*e));
        let products = Product::find().filter(product::Column::Id.is_in(values.clone()))
            .all(&self.db)
            .await
            .map_err(|_| ProductServiceErr::Internal)?;

        Product::delete_many()
            .filter(product::Column::Id.is_in(values))
            .exec(&self.db)
            .await
            .map(|_| products.into())
            .map_err(|_| ProductServiceErr::Internal)
    }
}
