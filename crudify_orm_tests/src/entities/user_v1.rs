use crudify_orm::Entity;
use sqlx::FromRow;

#[derive(Entity, FromRow)]
#[entity(table_name = "user_v1")]
pub struct UserV1Entity {
    #[entity(id)]
    pub id: i32,
}
