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
