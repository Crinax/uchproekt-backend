use sea_orm::prelude::*;
use sea_orm::sea_query::{Alias, IntoIden, SelectExpr, SelectStatement};
use sea_orm::{EntityTrait, FromQueryResult, QueryTrait};

pub struct Prefixer<S: QueryTrait<QueryStatement = SelectStatement>> {
    pub selector: S,
}
impl<S: QueryTrait<QueryStatement = SelectStatement>> Prefixer<S> {
    pub fn new(selector: S) -> Self {
        Self { selector }
    }
    pub fn add_columns<T: EntityTrait>(mut self, entity: T) -> Self {
        for col in <T::Column as sea_orm::entity::Iterable>::iter() {
            let alias = format!("{}{}", entity.table_name(), col.to_string()); // we use entity.table_name() as prefix
            self.selector.query().expr(SelectExpr {
                expr: col.select_as(col.into_expr()),
                alias: Some(Alias::new(&alias).into_iden()),
                window: None,
            });
        }
        self
    }
}

pub fn parse_query_to_model<M: FromQueryResult, E: EntityTrait>(
    res: &sea_orm::QueryResult,
) -> Result<M, sea_orm::DbErr> {
    M::from_query_result(res, E::default().table_name())
}
