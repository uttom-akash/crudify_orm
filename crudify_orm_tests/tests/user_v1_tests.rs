use crudify_orm_tests::entities::user_v1::*;
use sqlx::PgPool;
use rand::Rng;

#[tokio::test]
async fn test_user_v1_create() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = "postgres://postgres:postgres@127.0.0.1:5432/crudify_orm_test";

    let pool = PgPool::connect(database_url).await?;

    let id: i32 = rand::thread_rng().gen_range(1..=10_000);

    let user_create = UserV1EntityCreate { id};

    UserV1Entity::create(user_create, &pool).await?;

    Ok(())
}
