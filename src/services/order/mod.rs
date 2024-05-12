use std::collections::HashSet;

use entity::order::{self, Entity as Order};
use entity::product::{self, Entity as Product};
use entity::products_in_order::{self, Entity as ProductsInOrder};
use sea_orm::{
    ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set, TransactionTrait,
};
use serde::Serialize;

use crate::services::product::ProductSerializable;

#[derive(Clone, Debug)]
pub enum OrderInsertionErr {
    Internal,
    ProductNotFound(Vec<u32>),
}

pub struct OrderService {
    db: DatabaseConnection,
}

#[derive(Clone, Debug, Serialize)]
pub struct OrderSerializable {
    id: u32,
    name: String,
    surname: String,
    phone: String,
    address: String,
    products: Vec<ProductSerializable>,
}

impl From<(order::Model, Vec<product::Model>)> for OrderSerializable {
    fn from((model, products): (order::Model, Vec<product::Model>)) -> Self {
        Self {
            id: model.id as u32,
            name: model.name,
            surname: model.surname,
            phone: model.phone,
            address: model.address,
            products: products.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
struct OrderInsertion {
    id: u32,
}

impl OrderService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        name: String,
        surname: String,
        phone: String,
        address: String,
        products: Vec<u32>,
    ) -> Result<OrderInsertion, OrderInsertionErr> {
        let model = order::ActiveModel {
            name: Set(name),
            surname: Set(surname),
            phone: Set(phone),
            address: Set(address),
            ..Default::default()
        };

        let set_of_products: HashSet<u32> = products.clone().into_iter().collect();

        let not_found_products: Vec<u32> = Product::find()
            .filter(product::Column::Id.is_in(products.clone()))
            .all(&self.db)
            .await
            .map(|prdcts| {
                let result = prdcts
                    .iter()
                    .map(|product| product.id as u32)
                    .collect::<HashSet<u32>>();

                set_of_products.difference(&result).copied().collect()
            })
            .map_err(|_| OrderInsertionErr::Internal)?;

        if !not_found_products.is_empty() {
            return Err(OrderInsertionErr::ProductNotFound(not_found_products));
        }

        let insertion_result = self
            .db
            .transaction::<_, OrderInsertion, DbErr>(move |tx| {
                Box::pin(async move {
                    let insertion_result =
                        Order::insert(model)
                            .exec(tx)
                            .await
                            .map(|result| OrderInsertion {
                                id: result.last_insert_id as u32,
                            })?;

                    ProductsInOrder::insert_many(products.iter().map(|product| {
                        products_in_order::ActiveModel {
                            product_id: Set(*product as i32),
                            order_id: Set(insertion_result.id as i32),
                            ..Default::default()
                        }
                    }));

                    Ok(insertion_result)
                })
            })
            .await
            .map_err(|_| OrderInsertionErr::Internal)?;

        Ok(insertion_result)
    }
}
