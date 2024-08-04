use std::collections::{hash_map::Entry, HashMap, HashSet};

use entity::order::{self, Entity as Order};
use entity::product::{self, Entity as Product};
use entity::products_in_order::{self, Entity as ProductsInOrder};
use entity::{field, field_product};
use sea_orm::{
    ColumnTrait, DatabaseConnection, DbErr, EntityTrait, FromQueryResult, JoinType, QueryFilter,
    QuerySelect, RelationTrait, Set, TransactionTrait,
};
use serde::{Deserialize, Serialize};

use crate::{
    services::product::{ProductWithQuantityModel, ProductWithQuantitySerializable},
    utilities::seaorm_utils::{parse_query_to_model, Prefixer},
};

use super::product::{FieldWithValue, ProductSerializable, ProductWithQuantityWithFieldModel};

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
    pub product: ProductWithQuantityWithFieldModel,
}

impl FromQueryResult for OrderWithProductsModel {
    fn from_query_result(res: &sea_orm::prelude::QueryResult, _pre: &str) -> Result<Self, DbErr> {
        let order = parse_query_to_model::<order::Model, Order>(res)?;
        let products_in_order =
            parse_query_to_model::<products_in_order::Model, ProductsInOrder>(res)?;
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

        Ok(OrderWithProductsModel {
            id: order.id,
            name: order.name,
            surname: order.surname,
            phone: order.phone,
            address: order.address,
            product: ProductWithQuantityWithFieldModel {
                id: product.id,
                name: product.name,
                price: product.price,
                article: product.article,
                description: product.description,
                photo: product.photo,
                quantity: products_in_order.quantity,
                field: field_with_value,
            },
        })
    }
}

impl OrderService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    fn products_with_field_to_serializable(
        products: Vec<ProductWithQuantityWithFieldModel>,
    ) -> Vec<ProductWithQuantitySerializable> {
        let mut product_index_map: HashMap<u32, usize> = HashMap::new();
        let mut result: Vec<ProductWithQuantitySerializable> = Vec::new();

        for (index, product) in products.iter().enumerate() {
            let product_id = product.id as u32;

            if let Entry::Vacant(e) = product_index_map.entry(product_id) {
                result.push(ProductWithQuantitySerializable {
                    product: ProductSerializable {
                        id: product.id,
                        name: product.name.to_owned(),
                        price: product.price.to_owned(),
                        article: product.article.to_owned(),
                        description: product.description.to_owned(),
                        photo: product.photo,
                        fields: Vec::new(),
                    },
                    quantity: product.quantity as u32,
                });

                e.insert(index);

                if let Some(field) = &product.field {
                    result[index].product.fields.push(field.clone().into());
                }
            } else {
                let key = product_index_map.get(&(product.id as u32)).unwrap();

                if let Some(field) = &product.field {
                    result[*key].product.fields.push(field.clone().into());
                }
            }
        }

        return result;
    }

    pub async fn get_all(&self) -> Result<Vec<OrderSerializable>, OrderGetError> {
        let select = Order::find()
            .join(JoinType::LeftJoin, order::Relation::ProductsInOrder.def())
            .join(
                JoinType::LeftJoin,
                products_in_order::Relation::Product.def(),
            )
            .join(JoinType::LeftJoin, product::Relation::FieldProduct.def())
            .join(JoinType::LeftJoin, field_product::Relation::Field.def());

        let result = Prefixer::new(select)
            .add_columns(Order)
            .add_columns(ProductsInOrder)
            .add_columns(Product)
            .add_columns(field::Entity)
            .add_columns(field_product::Entity)
            .selector
            .into_model::<OrderWithProductsModel>()
            .all(&self.db)
            .await
            .map_err(|_| OrderGetError::Internal)?;

        let mut order_index_map: HashMap<i32, usize> = HashMap::new();
        let mut response: Vec<OrderSerializable> = Vec::new();
        let products: Vec<ProductWithQuantityWithFieldModel> =
            result.iter().map(|order| order.product.clone()).collect();
        let serializable_products: HashMap<i32, ProductWithQuantitySerializable> =
            Self::products_with_field_to_serializable(products)
                .into_iter()
                .map(|product| (product.product.id, product))
                .collect();

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

                response[index]
                    .products
                    .push(serializable_products[&order.product.id].clone());
            } else {
                let key = order_index_map.get(&order.id).unwrap();
                response[*key]
                    .products
                    .push(serializable_products[&order.product.id].clone());
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
