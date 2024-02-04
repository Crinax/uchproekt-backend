use std::collections::HashMap;

use sea_orm::{DatabaseConnection, EntityTrait, Order, QueryOrder};
use serde::Serialize;

use entity::category::{self, Entity as Category};

pub struct CategoryService {
    db: DatabaseConnection,
}

#[derive(Copy, Clone, Debug)]
pub enum CategoriesServiceErr {
    Internal,
    NotFound
}

#[derive(Serialize, Debug, Clone)]
pub struct CategorySerializable {
    id: u32,
    name: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct CategoryTreeSerializable {
    id: u32,
    name: String,
    categories: Vec<CategoryTreeSerializable>,
}

// impl CategoryTreeSerializable {
//     fn from_vec(categories: Vec<category::Model>) -> Vec<CategoryTreeSerializable> {
//         let mut inverse_link: HashMap<u32, Vec<CategoryTreeSerializable>> = HashMap::new();
//         let mut root_maping: HashMap<u32, CategoryTreeSerializable> = HashMap::new();
//
//         for category in categories {
//             if let Some(parent_id) = category.parent_id {
//                 let parent_id = parent_id as u32;
//
//                 if inverse_link.contains_key(&parent_id) {
//                     inverse_link.get_mut(&parent_id)
//                         .unwrap()
//                         .push(category.into())
//                 } else {
//                     inverse_link.insert(parent_id, vec![category.into()]);
//                 }
//             } else {
//                 let category_id = category.id as u32;
//
//                 root_maping.insert(category_id, category.into());
//             }
//         }
//
//         inverse_link.into_iter()
//             .map(move |(key, value)| {
//                 let mut root = root_maping.remove(&key).unwrap();
//
//                 root.categories = value;
//
//                 root
//             })
//             .collect()
//     }
// }

impl From<category::Model> for CategoryTreeSerializable {
    fn from(value: category::Model) -> Self {
        Self {
            id: value.id as u32,
            name: value.name,
            categories: vec![]
        }
    }
}

impl CategoryService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn all(&self) -> Result<Vec<CategoryTreeSerializable>, CategoriesServiceErr> {
        Category::find()
            .order_by(category::Column::Id, Order::Asc)
            .all(&self.db)
            .await
            .map(|models| {
                models.into_iter()
                    .map(Into::into)
                    .collect()
            })
            .map_err(|_| CategoriesServiceErr::Internal)
    }
}
