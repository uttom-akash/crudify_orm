#[cfg(test)]
mod tests {
    use chrono::{Duration, Utc};
    use crudify_orm_tests::entities::user_v2::*;
    use sqlx::PgPool;
    use uuid::Uuid;

    async fn setup_db() -> PgPool {
        // Ensure you have a `.env` with DATABASE_URL or use a test DB URL directly
        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:5432/crudify_orm_test".to_string()
        });

        PgPool::connect(&database_url)
            .await
            .expect("❌ Failed to connect to test database")
    }

    #[tokio::test]
    async fn test_create_user() {
        let pool = setup_db().await;

        let id: Uuid = Uuid::new_v4();

        let user_create_dto = UserV2EntityCreate {
            id,
            name: format!("test-user-{}", id),
            user_type: "test".to_string(),
            created_at: None,
            active: true,
        };

        let created = UserV2Entity::create(user_create_dto, &pool)
            .await
            .expect("❌ Failed to create user");

        assert_eq!(created.id, id);
        assert!(created.active);
        assert!(created.name.contains("test-user"));
    }

    #[tokio::test]
    async fn test_update_user() {
        let pool = setup_db().await;

        // First create
        let id: Uuid = Uuid::new_v4();

        let create_dto = UserV2EntityCreate {
            id,
            name: format!("test-update-{}", id),
            user_type: "test".to_string(),
            created_at: None,
            active: true,
        };

        let created = UserV2Entity::create(create_dto, &pool).await.unwrap();

        // Then update
        let update_dto = UserV2EntityUpdate {
            id: Some(created.id),
            name: Some(format!("updated-name-{}", id)),
            user_type: None,
            created_at: None,
            active: Some(false),
        };
        let updated = UserV2Entity::update_by_id(created.id, update_dto, &pool)
            .await
            .unwrap();

        assert_eq!(updated.active, false);
        assert_eq!(updated.name, format!("updated-name-{}", id));
    }

    #[tokio::test]
    async fn test_get_user_by_id() {
        let pool = setup_db().await;

        let id: Uuid = Uuid::new_v4();

        let create_dto = UserV2EntityCreate {
            id,
            name: format!("get-test-{}", id),
            user_type: "test".to_string(),
            created_at: None,
            active: true,
        };
        let created = UserV2Entity::create(create_dto, &pool).await.unwrap();

        let queried = UserV2Entity::get_by_id(created.id, &pool)
            .await
            .expect("❌ Failed to query by id")
            .expect("❌ user not found");

        assert_eq!(queried.id, created.id);
        assert_eq!(queried.name, created.name);
    }

    #[tokio::test]
    async fn test_keyset_pagination() {
        let pool = setup_db().await;

        let now = Utc::now().naive_utc();
        let created_at = now - Duration::hours(12);
        let id: Uuid = Uuid::new_v4();
        let limit: i64 = 5;

        let results = UserV2Entity::paginate_dby_id_created_at(
            PaginationCursorIdCreatedAt {
                id,
                created_at,
                limit,
                condition: ">".to_string(),
                order_by: "desc".to_string(),
            },
            &pool,
        )
        .await
        .expect("❌ Pagination failed");

        assert!(results.len() <= limit as usize);
    }

    #[tokio::test]
    async fn test_page_pagination() {
        let pool = setup_db().await;

        let results = UserV2Entity::get_paged(
            PagePagination {
                page: 1,
                page_size: 5,
            },
            &pool,
        )
        .await
        .expect("❌ Page pagination failed");

        assert!(results.len() <= 5);
    }

    #[tokio::test]
    async fn test_filter_users() {
        let pool = setup_db().await;

        let mut filter = UserV2EntityFilter::default();
        filter.active = Some(true);
        filter.active_condition = Some("=".to_string());

        let results = UserV2Entity::filter(filter, &pool)
            .await
            .expect("❌ Filter query failed");

        // We can't guarantee count, but we can assert it's a valid Vec
        assert!(results.iter().all(|p| p.active == true));
    }

    #[tokio::test]
    async fn test_delete_user() {
        let pool = setup_db().await;

        // Create a user
        let id: Uuid = Uuid::new_v4();

        let dto = UserV2EntityCreate {
            id,
            name: format!("delete-test-{}", id),
            user_type: "test".to_string(),
            created_at: None,
            active: true,
        };
        let created = UserV2Entity::create(dto, &pool).await.unwrap();

        // Delete
        UserV2Entity::delete_by_id(created.id, &pool)
            .await
            .expect("❌ Failed to delete user");

        // Verify deletion
        let queried = UserV2Entity::get_by_id(created.id, &pool)
            .await
            .expect("❌ Query after delete failed");

        assert!(queried.is_none(), "user should be deleted");
    }
}
