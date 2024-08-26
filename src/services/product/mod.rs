use std::collections::hash_map::Entry;
use std::collections::HashMap;

use migration::OnConflict;
use rust_decimal::Decimal;
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, FromQueryResult, JoinType, PaginatorTrait, QueryFilter, QuerySelect, RelationTrait, Set, TransactionTrait
};

use entity::product::{self, Entity as Product};
use entity::{field, field_product};
use serde::Serialize;
use uuid::Uuid;

use crate::api::FieldInProductDto;
use crate::utilities::seaorm_utils::{parse_query_to_model, Prefixer};

use super::field::field_type::FieldType;

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
    pub id: i32,
    pub name: String,
    pub price: rust_decimal::Decimal,
    pub article: String,
    pub description: String,
    pub photo: Option<Uuid>,
    pub fields: Vec<FieldInProduct>,
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
    pub fields: Vec<FieldInProduct>,
}

#[derive(Clone, Debug)]
pub struct ProductWithQuantityWithFieldModel {
    pub id: i32,
    pub name: String,
    pub price: Decimal,
    pub article: String,
    pub description: String,
    pub photo: Option<Uuid>,
    pub quantity: i32,
    pub field: Option<FieldWithValue>,
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
                fields: value.fields,
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
pub struct ProductInsertionUpdate {
    id: u32,
}

#[derive(Clone, Debug, Serialize)]
pub struct ProductAddFieldUpdate {
    product_id: u32,
    field_id: u32,
    value: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct FieldInProduct {
    id: u32,
    r#type: FieldType,
    value: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct FieldWithValue {
    pub id: u32,
    pub r#type: FieldType,
    pub value: String,
    pub product_id: u32,
}

impl From<FieldWithValue> for FieldInProduct {
    fn from(value: FieldWithValue) -> Self {
        FieldInProduct {
            id: value.id,
            r#type: value.r#type,
            value: value.value,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ProductWithField {
    pub id: i32,
    pub name: String,
    pub price: Decimal,
    pub article: String,
    pub description: String,
    pub photo: Option<Uuid>,
    pub field: Option<FieldWithValue>,
}

impl FromQueryResult for ProductWithField {
    fn from_query_result(res: &sea_orm::QueryResult, _pre: &str) -> Result<Self, sea_orm::DbErr> {
        let product = parse_query_to_model::<product::Model, Product>(res)?;
        let field_product =
            parse_query_to_model::<field_product::Model, field_product::Entity>(res).ok();

        let field = parse_query_to_model::<field::Model, field::Entity>(res).ok();

        let field_with_value = if let Some(field_product) = field_product {
            if let Some(field) = field {
                Some(FieldWithValue {
                    id: field.id as u32,
                    r#type: field.r#type.into(),
                    value: field_product.value,
                    product_id: field_product.product_id as u32,
                })
            } else {
                None
            }
        } else {
            None
        };

        Ok(ProductWithField {
            id: product.id,
            name: product.name,
            price: product.price,
            article: product.article,
            description: product.description,
            photo: product.photo,
            field: field_with_value,
        })
    }
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
            fields: Vec::new(),
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
            fields: Vec::new(),
        }
    }
}

impl ProductService {
    const MAX_PRODUCTS_PER_PAGE: u64 = 15;

    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub fn products_with_field_to_serializable(
        products: Vec<ProductWithField>,
    ) -> Vec<ProductSerializable> {
        let mut product_index_map: HashMap<u32, usize> = HashMap::new();
        let mut result: Vec<ProductSerializable> = Vec::new();

        for (index, product) in products.iter().enumerate() {
            let product_id = product.id as u32;

            if let Entry::Vacant(e) = product_index_map.entry(product_id) {
                result.push(ProductSerializable {
                    id: product.id,
                    name: product.name.to_owned(),
                    price: product.price.to_owned(),
                    article: product.article.to_owned(),
                    description: product.description.to_owned(),
                    photo: product.photo,
                    fields: Vec::new(),
                });

                e.insert(index);

                if let Some(field) = &product.field {
                    result[index].fields.push(field.clone().into());
                }
            } else {
                let key = product_index_map.get(&(product.id as u32)).unwrap();
                if let Some(field) = &product.field {
                    result[*key].fields.push(field.clone().into());
                }
            }
        }

        return result;
    }

    pub async fn all(&self, page: u64) -> Result<Vec<ProductSerializable>, ProductServiceErr> {
        let selector = Product::find()
            .join(JoinType::LeftJoin, product::Relation::FieldProduct.def())
            .join(JoinType::LeftJoin, field_product::Relation::Field.def());

        let products = Prefixer::new(selector)
            .add_columns(field::Entity)
            .add_columns(field_product::Entity)
            .add_columns(product::Entity)
            .selector
            .into_model::<ProductWithField>()
            .paginate(&self.db, ProductService::MAX_PRODUCTS_PER_PAGE)
            .fetch_page(page)
            .await
            .map_err(|_| ProductServiceErr::Internal)?;

        Ok(ProductService::products_with_field_to_serializable(
            products,
        ))
    }

    pub async fn update(
        &self,
        id: u32,
        name: &str,
        price: Decimal,
        article: &str,
        description: &str,
        photo: Option<Uuid>,
    ) -> Result<ProductInsertionUpdate, ProductServiceErr> {
        let model = product::ActiveModel {
            id: Set(id as i32),
            name: Set(name.to_owned()),
            price: Set(price),
            article: Set(article.to_owned()),
            description: Set(description.to_owned()),
            photo: Set(photo),
            ..Default::default()
        };

        Product::update(model)
            .exec(&self.db)
            .await
            .map_err(|err| match err {
                sea_orm::DbErr::RecordNotFound(_) => ProductServiceErr::NotFound,
                _ => ProductServiceErr::Internal,
            })
            .map(|_| ProductInsertionUpdate { id })
    }

    pub async fn remove_field_from_product(
        &self,
        product_id: u32,
        field_id: u32,
    ) -> Result<(), ProductServiceErr> {
        field_product::Entity::delete(field_product::ActiveModel {
            product_id: Set(product_id as i32),
            field_id: Set(field_id as i32),
            ..Default::default()
        })
        .exec(&self.db)
        .await
        .map_err(|err| match err {
            sea_orm::DbErr::RecordNotFound(_) => ProductServiceErr::NotFound,
            _ => ProductServiceErr::Internal,
        })?;

        Ok(())
    }

    pub async fn add_or_update_field_to_product(
        &self,
        product_id: u32,
        field_id: u32,
        value: &str,
    ) -> Result<ProductAddFieldUpdate, ProductServiceErr> {
        let model = field_product::ActiveModel {
            product_id: Set(product_id as i32),
            field_id: Set(field_id as i32),
            value: Set(value.to_owned()),
        };
        field_product::Entity::insert(model)
            .on_conflict(
                OnConflict::columns([
                    field_product::Column::ProductId,
                    field_product::Column::FieldId,
                ])
                .update_column(field_product::Column::Value)
                .to_owned(),
            )
            .exec(&self.db)
            .await
            .map_err(|_| ProductServiceErr::Internal)
            .map(|_| ProductAddFieldUpdate {
                product_id,
                field_id,
                value: value.to_owned(),
            })
    }

    pub async fn create(
        &self,
        name: String,
        price: Decimal,
        article: String,
        description: String,
        photo: Option<Uuid>,
        fields: Vec<FieldInProductDto>,
    ) -> Result<ProductInsertionUpdate, ProductServiceErr> {
        let transaction = self
            .db
            .begin()
            .await
            .map_err(|_| ProductServiceErr::Internal)?;

        let result = Product::insert(product::ActiveModel {
            name: Set(name),
            price: Set(price),
            article: Set(article),
            description: Set(description),
            photo: Set(photo),
            ..Default::default()
        })
        .exec(&transaction)
        .await
        .map(|result| ProductInsertionUpdate {
            id: result.last_insert_id as u32,
        })
        .map_err(|_| ProductServiceErr::Internal)?;

        field_product::Entity::insert_many(fields.iter().map(|f| field_product::ActiveModel {
            product_id: Set(result.id as i32),
            field_id: Set(f.id as i32),
            value: Set(f.value.to_owned()),
        }))
        .on_conflict(
            OnConflict::columns([
                field_product::Column::ProductId,
                field_product::Column::FieldId,
            ])
            .update_column(field_product::Column::Value)
            .to_owned(),
        )
        .exec(&transaction)
        .await
        .map_err(|_| ProductServiceErr::Internal)?;

        transaction
            .commit()
            .await
            .map_err(|_| ProductServiceErr::Internal)?;

        Ok(result)
    }

    pub async fn get(&self, id: u32) -> Result<ProductSerializable, ProductServiceErr> {
        let selector = Product::find_by_id(id as i32)
            .join(JoinType::LeftJoin, product::Relation::FieldProduct.def())
            .join(JoinType::LeftJoin, field_product::Relation::Field.def());

        let product = Prefixer::new(selector)
            .add_columns(field::Entity)
            .add_columns(field_product::Entity)
            .add_columns(product::Entity)
            .selector
            .into_model::<ProductWithField>()
            .all(&self.db)
            .await
            .map_err(|err| {
                log::error!("{:?}", err);
                ProductServiceErr::Internal
            })?;

        if product.len() == 0 {
            return Err(ProductServiceErr::NotFound);
        }

        let seriallizable = ProductService::products_with_field_to_serializable(product);

        Ok(seriallizable[0].clone())
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
