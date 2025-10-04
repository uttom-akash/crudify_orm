use chrono::NaiveDateTime;
use crudify_orm::Entity;
use sqlx::FromRow;

#[derive(Entity, FromRow)]
#[entity(table_name = "user_v2")]
pub struct UserV2Entity {
    #[entity(id, keyset_pagination("id_created_at"))]
    pub id: uuid::Uuid,

    #[entity(alias = "user_name")]
    pub name: String,

    pub user_type: String,

    #[entity(default, keyset_pagination("id_created_at"))]
    pub created_at: NaiveDateTime,

    pub active: bool,
}
