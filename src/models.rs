use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::processed_block)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Block {
    pub id: i32,
    pub block_number: i64,
}
