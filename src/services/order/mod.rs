use std::collections::{hash_map::Entry, HashMap, HashSet};

use entity::order::{self, Entity as Order};
use entity::product::{self, Entity as Product};
use entity::products_in_order::{self, Entity as ProductsInOrder};
use sea_orm::{
    ColumnTrait, DatabaseConnection, DbErr, EntityTrait, FromQueryResult, JoinType, QueryFilter,
    QuerySelect, RelationTrait, Set, TransactionTrait,
};
use serde::{Deserialize, Serialize};

use crate::{
    services::product::{ProductWithQuantityModel, ProductWithQuantitySerializable},
    utilities::seaorm_utils::{parse_query_to_model, Prefixer},
};

#[derive(Clone, Debug)]
pub enum OrderInsertionErr {
    Internal,
    ProductNotFound(Vec<u32>),
}

#[derive(Clone, Debug)]
pub enum OrderGetError {
    Internal,
}

pub struct OrderService {
    db: DatabaseConnection,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ProductWithQuantity {
    id: u32,
    quantity: u32,
}

#[derive(Clone, Debug, Serialize)]
pub struct OrderSerializable {
    id: u32,
    name: String,
    surname: String,
    phone: String,
    address: String,
    products: Vec<ProductWithQuantitySerializable>,
}

#[derive(Clone, Debug, Serialize)]
pub struct OrderInsertion {
    id: u32,
}

#[derive(Clone, Debug)]
struct OrderWithProductsModel {
    pub id: i32,
    pub name: String,
    pub surname: String,
    pub phone: String,
    pub address: String,
    pub product: ProductWithQuantityModel,
}

impl FromQueryResult for OrderWithProductsModel {
    fn from_query_result(res: &sea_orm::prelude::QueryResult, _pre: &str) -> Result<Self, DbErr> {
        let order = parse_query_to_model::<order::Model, Order>(res)?;
        let products_in_order =
            parse_query_to_model::<products_in_order::Model, ProductsInOrder>(res)?;
        let product = parse_query_to_model::<product::Model, Product>(res)?;

        Ok(OrderWithProductsModel {
            id: order.id,
            name: order.name,
            surname: order.surname,
            phone: order.phone,
            address: order.address,
            product: ProductWithQuantityModel {
                id: product.id,
                name: product.name,
                price: product.price,
                article: product.article,
                description: product.description,
                photo: product.photo,
                quantity: products_in_order.quantity,
            },
        })
    }
}

impl OrderService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn get_all(&self) -> Result<Vec<OrderSerializable>, OrderGetError> {
        let select = Order::find()
            .join(JoinType::LeftJoin, order::Relation::ProductsInOrder.def())
            .join(
                JoinType::LeftJoin,
                products_in_order::Relation::Product.def(),
            );

        let result = Prefixer::new(select)
            .add_columns(Order)
            .add_columns(ProductsInOrder)
            .add_columns(Product)
            .selector
            .into_model::<OrderWithProductsModel>()
            .all(&self.db)
            .await
            .map_err(|_| OrderGetError::Internal)?;

        let mut order_index_map: HashMap<i32, usize> = HashMap::new();
        let mut response: Vec<OrderSerializable> = Vec::new();

        for (index, order) in result.iter().enumerate() {
            if let Entry::Vacant(e) = order_index_map.entry(order.id) {
                response.push(OrderSerializable {
                    id: order.id as u32,
                    name: order.name.clone(),
                    surname: order.surname.clone(),
                    phone: order.phone.clone(),
                    address: order.address.clone(),
                    products: Vec::new(),
                });

                e.insert(index);

                response[index].products.push(order.product.clone().into())
            } else {
                let key = order_index_map.get(&order.id).unwrap();
                response[*key].products.push(order.product.clone().into());
            }
        }

        Ok(response)
    }

    pub async fn create(
        &self,
        name: String,
        surname: String,
        phone: String,
        address: String,
        products: Vec<ProductWithQuantity>,
    ) -> Result<OrderInsertion, OrderInsertionErr> {
        let model = order::ActiveModel {
            name: Set(name),
            surname: Set(surname),
            phone: Set(phone),
            address: Set(address),
            ..Default::default()
        };

        let set_of_products: HashMap<u32, u32> = products
            .clone()
            .into_iter()
            .map(|pr| (pr.id, pr.quantity))
            .collect();

        let ids: HashSet<u32> = set_of_products.keys().copied().collect();

        let not_found_products: Vec<u32> = Product::find()
            .filter(product::Column::Id.is_in(ids.clone()))
            .all(&self.db)
            .await
            .map(|prdcts| {
                let result = prdcts
                    .iter()
                    .map(|product| product.id as u32)
                    .collect::<HashSet<u32>>();

                ids.difference(&result).copied().collect()
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
                            product_id: Set(product.id as i32),
                            order_id: Set(insertion_result.id as i32),
                            quantity: Set(product.quantity as i32),
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
