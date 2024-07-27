use rust_decimal::Decimal;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

use entity::product::{self, Entity as Product};
use serde::Serialize;
use uuid::Uuid;

pub struct ProductService {
    db: DatabaseConnection,
}

#[derive(Copy, Clone, Debug)]
pub enum ProductServiceErr {
    Internal,
    NotFound,
}

#[derive(Clone, Debug, Serialize)]
pub struct ProductSerializable {
    id: i32,
    name: String,
    price: rust_decimal::Decimal,
    article: String,
    description: String,
    photo: Option<Uuid>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ProductWithQuantitySerializable {
    pub product: ProductSerializable,
    pub quantity: u32,
}

#[derive(Clone, Debug)]
pub struct ProductWithQuantityModel {
    pub id: i32,
    pub name: String,
    pub price: Decimal,
    pub article: String,
    pub description: String,
    pub photo: Option<Uuid>,
    pub quantity: i32,
}

impl From<ProductWithQuantityModel> for ProductWithQuantitySerializable {
    fn from(value: ProductWithQuantityModel) -> Self {
        ProductWithQuantitySerializable {
            product: ProductSerializable {
                id: value.id,
                name: value.name,
                price: value.price,
                article: value.article,
                description: value.description,
                photo: value.photo,
            },
            quantity: value.quantity as u32,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ProductIdx {
    idx: Vec<u32>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ProductInsertion {
    id: u32,
}

impl From<&[u32]> for ProductIdx {
    fn from(value: &[u32]) -> Self {
        ProductIdx {
            idx: value.to_vec(),
        }
    }
}

impl From<Vec<product::Model>> for ProductIdx {
    fn from(value: Vec<product::Model>) -> Self {
        ProductIdx {
            idx: value.into_iter().map(|v| v.id as u32).collect(),
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

impl From<&product::Model> for ProductSerializable {
    fn from(model: &product::Model) -> Self {
        ProductSerializable {
            id: model.id,
            name: model.name.to_owned(),
            price: model.price.to_owned(),
            article: model.article.to_owned(),
            description: model.description.to_owned(),
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
            .map(|models| models.into_iter().map(Into::into).collect())
            .map_err(|_| ProductServiceErr::Internal)
    }

    pub async fn create(
        &self,
        name: String,
        price: Decimal,
        article: String,
        description: String,
        photo: Option<Uuid>,
    ) -> Result<ProductInsertion, ProductServiceErr> {
        Product::insert(product::ActiveModel {
            name: Set(name),
            price: Set(price),
            article: Set(article),
            description: Set(description),
            photo: Set(photo),
            ..Default::default()
        })
        .exec(&self.db)
        .await
        .map(|result| ProductInsertion {
            id: result.last_insert_id as u32,
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
        let products = Product::find()
            .filter(product::Column::Id.is_in(values.clone()))
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
