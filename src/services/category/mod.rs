use std::{cell::RefCell, collections::HashMap, rc::Rc};

use sea_orm::{DatabaseConnection, EntityTrait, Order, QueryOrder, Set};
use serde::Serialize;

use entity::category::{self, Entity as Category};

pub struct CategoryService {
    db: DatabaseConnection,
}

#[derive(Copy, Clone, Debug)]
pub enum CategoriesServiceErr {
    Internal,
    AlreadyExists,
    InvalidParentId,
    NotFound
}

#[derive(Serialize, Debug, Clone)]
pub struct CategorySerializable {
    id: u32,
    name: String,
    parent_id: Option<u32>
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

// need to rewrite
impl CategoryTreeSerializable {
    fn from_vec(categories: Vec<category::Model>) -> Vec<CategoryTreeSerializable> {
        let mut result: Vec<CategoryTreeSerializable> = vec![];
        let mut id_mappings: HashMap<i32, CategoryTreeSerializable> = HashMap::new();

        for value in categories {
            let mut child: CategoryTreeSerializable = value.clone().into();

            if value.parent_id == None {
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

                id_mappings.get(&parent_id).map(
                    |v| v.categories.borrow_mut().push(child)
                );
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

impl From<category::Model> for CategoryTreeSerializable {
    fn from(value: category::Model) -> Self {
        Self {
            id: value.id as u32,
            name: value.name,
            categories: Rc::new(RefCell::new(vec![]))
        }
    }
}

impl From<&category::Model> for CategoryTreeSerializable {
    fn from(value: &category::Model) -> Self {
        Self {
            id: value.id as u32,
            name: value.name.clone(),
            categories: Rc::new(RefCell::new(vec![]))
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

    pub async fn all_tree(&self) -> Result<Vec<CategoryTreeSerializable>, CategoriesServiceErr> {
        Category::find()
            .order_by(category::Column::Id, Order::Asc)
            .all(&self.db)
            .await
            .map(|models| CategoryTreeSerializable::from_vec(models))
            .map_err(|_| CategoriesServiceErr::Internal)
    }

    pub async fn create(&self, name: &str, parent_id: Option<u32>) -> Result<CategoryInsertion, CategoriesServiceErr> {
        let category = category::ActiveModel {
            name: Set(name.to_owned()),
            parent_id: Set(parent_id.map(|v| v as i32)),
            ..Default::default()
        };

        Category::insert(category)
            .exec(&self.db)
            .await
            .map(|model| CategoryInsertion { id: model.last_insert_id as u32 })
            .map_err(|err| {
                match err {
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
                    _ => CategoriesServiceErr::Internal
                }
            })
    }
}
