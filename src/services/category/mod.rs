use std::{cell::RefCell, collections::HashMap, rc::Rc};

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, Order, QueryFilter, QueryOrder,
    Set,
};
use serde::Serialize;

use entity::category::{self, Entity as Category};
use entity::product::{self, Entity as Product};

use crate::{services::product::ProductSerializable, utilities::serde_utils::Patch};

pub struct CategoryService {
    db: DatabaseConnection,
}

#[derive(Copy, Clone, Debug)]
pub enum CategoriesServiceErr {
    Internal,
    AlreadyExists,
    InvalidParentId,
    NotFound,
}

#[derive(Serialize, Debug, Clone)]
pub struct CategorySerializable {
    id: u32,
    name: String,
    parent_id: Option<u32>,
}

#[derive(Serialize, Debug, Clone)]
pub struct CategoryWithProductsSerializable {
    id: u32,
    name: String,
    parent_id: Option<u32>,
    products: Vec<ProductSerializable>,
}

#[derive(Serialize, Debug, Clone)]
pub struct CategoryInsertion {
    id: u32,
}

#[derive(Serialize, Debug, Clone)]
pub struct CategoryTreeSerializable {
    id: u32,
    name: String,
    categories: Rc<RefCell<Vec<CategoryTreeSerializable>>>,
}

#[derive(Serialize, Debug, Clone)]
pub struct CategoriesIdx {
    idx: Vec<u32>,
}

impl From<Vec<category::Model>> for CategoriesIdx {
    fn from(value: Vec<category::Model>) -> Self {
        Self {
            idx: value
                .into_iter()
                .map(|category| category.id as u32)
                .collect(),
        }
    }
}

// need to rewrite
impl CategoryTreeSerializable {
    fn from_vec(categories: Vec<category::Model>) -> Vec<CategoryTreeSerializable> {
        let mut result: Vec<CategoryTreeSerializable> = vec![];
        let mut id_mappings: HashMap<i32, CategoryTreeSerializable> = HashMap::new();

        for value in categories {
            let mut child: CategoryTreeSerializable = value.clone().into();

            if value.parent_id.is_none() {
                if id_mappings.contains_key(&value.id) {
                    child.categories = id_mappings[&value.id].categories.clone();
                }

                id_mappings.insert(value.id, child.clone());
                result.push(child);
                continue;
            }

            if let Some(parent_id) = value.parent_id {
                if !id_mappings.contains_key(&parent_id) {
                    let fake_root = CategoryTreeSerializable {
                        id: value.parent_id.map(|v| v as u32).unwrap(),
                        name: "".to_owned(),
                        categories: Rc::new(RefCell::new(vec![child.clone()])),
                    };

                    id_mappings.insert(value.id, child.clone());
                    id_mappings.insert(parent_id, fake_root);
                    continue;
                }

                id_mappings.insert(value.id, child.clone());

                if let Some(v) = id_mappings.get(&parent_id) {
                    v.categories.borrow_mut().push(child)
                }
            }
        }

        result
    }
}

impl From<category::Model> for CategorySerializable {
    fn from(value: category::Model) -> Self {
        Self {
            id: value.id as u32,
            name: value.name,
            parent_id: value.parent_id.map(|v| v as u32),
        }
    }
}

impl From<(category::Model, Vec<product::Model>)> for CategoryWithProductsSerializable {
    fn from(value: (category::Model, Vec<product::Model>)) -> Self {
        Self {
            id: value.0.id as u32,
            name: value.0.name,
            parent_id: value.0.parent_id.map(|v| v as u32),
            products: value.1.iter().map(Into::into).collect(),
        }
    }
}

impl From<category::ActiveModel> for CategorySerializable {
    fn from(value: category::ActiveModel) -> Self {
        Self {
            id: value.id.unwrap() as u32,
            name: value.name.unwrap(),
            parent_id: value.parent_id.unwrap().map(|v| v as u32),
        }
    }
}

impl From<category::Model> for CategoryTreeSerializable {
    fn from(value: category::Model) -> Self {
        Self {
            id: value.id as u32,
            name: value.name,
            categories: Rc::new(RefCell::new(vec![])),
        }
    }
}

impl From<&category::Model> for CategoryTreeSerializable {
    fn from(value: &category::Model) -> Self {
        Self {
            id: value.id as u32,
            name: value.name.clone(),
            categories: Rc::new(RefCell::new(vec![])),
        }
    }
}

impl CategoryService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn all(&self) -> Result<Vec<CategorySerializable>, CategoriesServiceErr> {
        Category::find()
            .order_by(category::Column::Id, Order::Asc)
            .all(&self.db)
            .await
            .map(|models| models.into_iter().map(Into::into).collect())
            .map_err(|_| CategoriesServiceErr::Internal)
    }

    pub async fn category_with_products(
        &self,
        id: u32,
    ) -> Result<CategoryWithProductsSerializable, CategoriesServiceErr> {
        Category::find_by_id(id as i32)
            .find_with_related(Product)
            .all(&self.db)
            .await
            .map_err(|_| CategoriesServiceErr::Internal)
            .map(|mut cat| {
                if cat.is_empty() {
                    return Err(CategoriesServiceErr::NotFound);
                }

                Ok(cat.remove(0).into())
            })?
    }

    pub async fn all_tree(&self) -> Result<Vec<CategoryTreeSerializable>, CategoriesServiceErr> {
        Category::find()
            .order_by(category::Column::Id, Order::Asc)
            .all(&self.db)
            .await
            .map(CategoryTreeSerializable::from_vec)
            .map_err(|_| CategoriesServiceErr::Internal)
    }

    pub async fn create(
        &self,
        name: &str,
        parent_id: Option<u32>,
    ) -> Result<CategoryInsertion, CategoriesServiceErr> {
        let category = category::ActiveModel {
            name: Set(name.to_owned()),
            parent_id: Set(parent_id.map(|v| v as i32)),
            ..Default::default()
        };

        Category::insert(category)
            .exec(&self.db)
            .await
            .map(|model| CategoryInsertion {
                id: model.last_insert_id as u32,
            })
            .map_err(|err| match err {
                sea_orm::DbErr::RecordNotInserted => CategoriesServiceErr::AlreadyExists,
                sea_orm::DbErr::Query(sea_orm::RuntimeErr::SqlxError(err)) => {
                    let database_error = err.as_database_error();

                    if database_error.is_none() {
                        return CategoriesServiceErr::Internal;
                    }

                    let database_error = database_error.unwrap();

                    if database_error.is_foreign_key_violation() {
                        return CategoriesServiceErr::InvalidParentId;
                    }

                    CategoriesServiceErr::Internal
                }
                _ => CategoriesServiceErr::Internal,
            })
    }

    pub async fn update(
        &self,
        id: u32,
        new_name: Option<&str>,
        parent_id: Patch<u32>,
    ) -> Result<CategorySerializable, CategoriesServiceErr> {
        let category = Category::find_by_id(id as i32)
            .one(&self.db)
            .await
            .map_err(|_| CategoriesServiceErr::Internal)?;

        if category.is_none() {
            return Err(CategoriesServiceErr::NotFound);
        }

        let mut category: category::ActiveModel = category.unwrap().into();

        if let Some(new_name) = new_name {
            category.name = Set(new_name.to_owned());
        }

        log::info!("{parent_id:?}");

        if let Patch::Null = parent_id {
            category.parent_id = Set(None);
        }

        if let Patch::Value(parent_id) = parent_id {
            category.parent_id = Set(Some(parent_id as i32));
        }

        category
            .save(&self.db)
            .await
            .map(Into::into)
            .map_err(|err| match err {
                sea_orm::DbErr::RecordNotInserted => CategoriesServiceErr::AlreadyExists,
                sea_orm::DbErr::Query(sea_orm::RuntimeErr::SqlxError(err)) => {
                    let database_error = err.as_database_error();

                    if database_error.is_none() {
                        return CategoriesServiceErr::Internal;
                    }

                    let database_error = database_error.unwrap();

                    if database_error.is_foreign_key_violation() {
                        return CategoriesServiceErr::InvalidParentId;
                    }

                    CategoriesServiceErr::Internal
                }
                _ => CategoriesServiceErr::Internal,
            })
    }

    pub async fn delete(&self, idx: &[u32]) -> Result<CategoriesIdx, CategoriesServiceErr> {
        let values = idx.iter().map(|v| Into::<sea_orm::Value>::into(*v));

        let categories = Category::find()
            .filter(category::Column::Id.is_in(values.clone()))
            .all(&self.db)
            .await
            .map_err(|_| CategoriesServiceErr::Internal)?;

        Category::delete_many()
            .filter(category::Column::Id.is_in(values))
            .exec(&self.db)
            .await
            .map(|_| categories.into())
            .map_err(|_| CategoriesServiceErr::Internal)
    }
}
