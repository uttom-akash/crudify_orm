# crudify-orm

**crudify-orm** automatically generates CRUD methods, DTOs, and pagination helpers for your database entities using the Entity derive macro.

Available as a Rust crate: [crates.io/crates/crudify_orm](https://crates.io/crates/crudify_orm)

## ✨ Features

- 🆕 **Create:** Automatically generate `create` methods.
- 🔍 **Read:** Fetch records by primary key.
- ✏️ **Update:** Auto-generate update methods with partial updates.
- ❌ **Delete:** Delete records by primary key.
- 📄 **Page Pagination:** Simple page-based pagination.
- 🔑 **Keyset Pagination:** Efficient cursor-based pagination for large datasets.
- 🔎 **Filtering:** Auto-generate filtering methods based on entity fields.
- 🗂 **DTO Generation:** Automatically generate DTOs for all CRUD operations.

---

## 📝 Note 
This crate currently has a hard dependency on sqlx:
```
sqlx = { version = "0.8.2", features = ["postgres", "runtime-tokio-rustls", "chrono"] }
```
Future versions may support other database backends.

⚠️ Important:
Since SQLx connects to the database at compile time for query validation, you must provide a valid connection string in your .env file:
```
DATABASE_URL=postgres://user:password@localhost:5432/your_database
```

you will find working examples here: 
[demo project](https://github.com/uttom-akash/crudify_orm/tree/feat/crux-x/examples/crud-operations)

## 1️⃣ Define Your Entity

```
use crudify_orm::Entity;
use sqlx::FromRow;

#[derive(Debug, Entity, FromRow)]
#[entity(table_name = "partners")]
struct PartnerEntity {
    #[entity(id, keyset_pagination("id_created_at"))]
    id: i64,

    #[entity(alias = "partner_name")]
    name: String,

    partner_type: String,

    #[entity(default, keyset_pagination("id_created_at"))]
    created_at: NaiveDateTime,

    enabled: bool,
}
```

### Explanation:

- `#[derive(Entity, FromRow)]` generates all CRUD and pagination methods.
- `#[entity(table_name = "...")]` specifies the database table.
- ##### Field attributes:

    - `id` marks the primary key.
    - `alias` maps struct field names to database column names.
    - `default` sets default values when creating a new record.
    - `keyset_pagination` specifies columns used for keyset pagination.

---

## 2️⃣ Create a Record
```
let partner_create_dto = PartnerEntityCreate {
    id: 1,
    name: "Test Partner".to_string(),
    partner_type: "test".to_string(),
    created_at: None,
    enabled: true,
};

let created_partner = PartnerEntity::create(partner_create_dto, &pool).await?;
println!("Created partner: {:?}", created_partner);
```

### Explanation:

- `PartnerEntityCreate` is auto-generated.
- `PartnerEntity::create` inserts the record into the database and returns the inserted row.

---

## 3️⃣ Read a Record

```
let queried_partner = PartnerEntity::get_by_id(created_partner.id, &pool).await?;
println!("Queried partner: {:?}", queried_partner);
```

Explanation:

- `get_by_id` fetches a record by its primary key.
- `Returns Option<PartnerEntity>`.

## 4️⃣ Update a Record
```
let partner_update_dto = PartnerEntityUpdate {
    id: Some(created_partner.id),
    name: Some("Updated Partner".to_string()),
    partner_type: None,
    created_at: None,
    enabled: Some(false),
};

let updated_partner = PartnerEntity::update_by_id(created_partner.id, partner_update_dto, &pool).await?;
println!("Updated partner: {:?}", updated_partner);
```

### Explanation:

- `PartnerEntityUpdate` is auto-generated and allows partial updates.
- Only fields set to `Some(value)` will be updated in the database.

---

## 5️⃣ Delete a Record
```
PartnerEntity::delete_by_id(updated_partner.id, &pool).await?;
println!("Partner deleted successfully");
```

### Explanation:

- `delete_by_id` deletes a record using its primary key.


## 6️⃣ Page Pagination
```
let page_results = PartnerEntity::get_paged(
    PagePagination { page: 1, page_size: 5 },
    &pool,
).await?;
println!("Page results: {:?}", page_results);
```

### Explanation:

- Simple pagination based on page and page_size.

- Returns a `Vec<PartnerEntity>` for the requested page.

---

## 7️⃣ Keyset Pagination

```
let mut id_cursor: i64 = i64::MAX;
let mut created_at = Utc::now().naive_utc();
let limit: i64 = 5;

loop {
    let results = PartnerEntity::paginate_dby_id_created_at(
        PaginationCursorIdCreatedAt {
            id: id_cursor,
            created_at,
            limit,
            condition: "<".to_string(),
            order_by: "desc".to_string(),
        },
        &pool,
    ).await?;

    if results.is_empty() { break; }

    println!("Keyset results: {:?}", results);

    // Update cursor for next iteration
    let last = results.last().unwrap();
    id_cursor = last.id;
    created_at = last.created_at;
}
```

### Explanation:

- **Keyset pagination** is more efficient for large datasets.
- Uses `id` and `created_at` as cursors to fetch the next set of rows.

## 8️⃣ Filter Records
```
let mut filter = PartnerEntityFilter::default();
filter.enabled = Some(false);
filter.enabled_condition = Some("=".to_string());
filter.id = Some(100);
filter.id_condition = Some("<=".to_string());

let filtered_results = PartnerEntity::filter(filter, &pool).await?;
println!("Filtered results: {:?}", filtered_results);
```

### Explanation:

- `PartnerEntityFilter` allows filtering using field conditions.
- Conditions can include `=, !=, <, >, <=, >=`.



## Full Example
```

use chrono::{Duration, NaiveDateTime, Utc};
use crudify_orm::Entity;
use rand::Rng;
use sqlx::{FromRow, PgPool};

#[derive(Debug,Entity, FromRow)]
#[entity(table_name = "partners")]
struct PartnerEntity {
    #[entity(id, keyset_pagination("id_created_at"))]
    id: i64,

    #[entity(alias = "partner_name")]
    name: String,

    partner_type: String,

    #[entity(default, keyset_pagination("id_created_at"))]
    created_at: NaiveDateTime,

    enabled: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = "postgres://crudx:crudx@127.0.0.1:5432/test_pagination_sqlx";

    // Create a connection pool
    let pool = PgPool::connect(database_url).await?;

    let id: i64 = rand::thread_rng().gen_range(1..=10000);

    let updated_id: i64 = rand::thread_rng().gen_range(10000..=100000);

    /// Auto generated create struct
    let partner_create_dto = PartnerEntityCreate {
        id: id.clone(),
        name: format!("akash-test partner-{}", id),
        partner_type: "test".to_string(),
        created_at: None,
        enabled: true,
    };

    /// Auto generated create method
    let created_partner = PartnerEntity::create(partner_create_dto, &pool).await?;

    println!(
        "########### created partner: {:?} ###########",
        created_partner
    );

    /// Auto generated update struct
    let partner_update_dto = PartnerEntityUpdate {
        id: Some(updated_id.clone()),
        name: Some(format!("updated partner-{}", updated_id)),
        partner_type: None,
        created_at: None,
        enabled: Some(false),
    };

    /// Auto generated update method
    let updated_partner =
        PartnerEntity::update_by_id(created_partner.id, partner_update_dto, &pool).await?;

    println!(
        "########### updated partner: {:?} ###########",
        updated_partner
    );

    /// Auto generated get by id method
    let queried_partner = PartnerEntity::get_by_id(updated_partner.id, &pool).await?;

    println!(
        "########### queried partner: {:?} ###########",
        queried_partner
    );


    /// Auto generated pagination
    println!("########### Pagination Results: ###########");
    let now = Utc::now().naive_utc();
    let mut created_at = now - Duration::hours(12);
    let mut id: i64 = i64::MAX;
    let limit: i64 = 6;

    loop {
        let results: Vec<PartnerEntity> = PartnerEntity::paginate_dby_id_created_at(
            PaginationCursorIdCreatedAt {
                id,
                created_at,
                limit,
                condition: "<".to_string(),
                order_by: "desc".to_string(),
            },
            &pool,
        )
        .await?;

        println!("### {:?} : {:?} ####", id, results);

        match results.last() {
            Some(partner_dbo) => {
                id = partner_dbo.id;
                created_at = partner_dbo.created_at;
            }
            None => break,
        };
    }

    /// Auto generated page pagination struct and method
    println!("########### Page Pagination Results: ###########");
    let results: Vec<PartnerEntity> = PartnerEntity::get_paged(
        PagePagination {
            page: 1,
            page_size: 6,
        },
        &pool,
    )
    .await?;

    println!("### {:?} ####", results.last());

    println!("### ---------------- ####");

    println!("#### FILTER ####");

    /// Auto generated filter struct and method
    let mut filter = PartnerEntityFilter::default();
    filter.id =  Some(id);
    filter.id_condition = Some("<=".to_string());
    filter.enabled = Some(false);
    filter.enabled_condition = Some("=".to_string());

    let results = PartnerEntity::filter(filter, &pool,).await?;
    
    println!("###### {:?} #####", results);


    /// Auto generated delete by id method
    match PartnerEntity::delete_by_id(updated_partner.id, &pool).await {
        Ok(_) => println!("########### Good to go: partner deleted ###########"),
        Err(e) => println!("########### ERROR: deleting partner: {} ###########", e),
    }

    match PartnerEntity::get_by_id(updated_partner.id, &pool).await {
        Ok(Some(_)) => println!("########### ERROR: partner found ###########"),
        _ => println!("########### Good to go: partner should not be found ###########"),
    }

    Ok(())
}

```

