use crate::{input::{ FieldDefinition, ObjectDefinition, PaginationMetadata}, utils::create_ident};
use quote::quote;
use syn::Ident;


/// CREATE methods
/// 
/// 
pub fn create_query_method(object: &ObjectDefinition) -> proc_macro2::TokenStream {
    let (columns, binds) = generate_create_query_columns_and_binds(object);
    let create_dto_struct_ident = object.get_create_dto_struct_ident();
    let struct_ident = object.get_struct_ident();

    let select_clause = format!(" RETURNING {}", get_all_select_clause(object));
    let insert_statement = format!("INSERT INTO {} (", object.table_name);

    quote! {
        pub async fn create<'a>(
            payload: #create_dto_struct_ident,
            pool: &::sqlx::PgPool,
        ) -> Result<#struct_ident, sqlx::Error>
        {
            let mut query_builder = sqlx::QueryBuilder::<sqlx::Postgres>::new(#insert_statement);

            #(#columns)*

            query_builder.push(") VALUES (");

            let mut separated = query_builder.separated(", ");
            #(#binds)*
            separated.push_unseparated(") ");
            // query_builder.push(")");

            query_builder.push(#select_clause);

            // Build and execute the query
            let result = query_builder.build_query_as::<#struct_ident>().fetch_one(pool).await?;

            Ok(result)
        }
    }
}

fn generate_create_query_columns_and_binds(
    object: &ObjectDefinition,
) -> (Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>) {
    let mut columns = vec![];
    let mut binds = vec![];
    let mut comma = false;

    for field in object.fields.iter() {
        let field_ident = field.name.clone();
        let mut field_name_str = field.column_name.to_string();

        if comma {
            field_name_str = format!(", {}", field_name_str);
        }

        match field.has_default {
            true => {
                columns.push(quote! {
                    if payload.#field_ident.is_some() {
                        query_builder.push(#field_name_str);
                    }
                });
                binds.push(quote! {
                    if payload.#field_ident.is_some() {
                        separated.push_bind(payload.#field_ident);
                    }
                })
            }
            false => {
                columns.push(quote! {
                    query_builder.push(#field_name_str);
                });
                binds.push(quote! {
                    separated.push_bind(payload.#field_ident);
                })
            }
        }
        comma = true;
    }

    (columns, binds)
}



/// GET methods
/// 
/// 
pub fn get_query_method(object_definitions: &ObjectDefinition) -> proc_macro2::TokenStream {
    let read_query = generate_read_query(&object_definitions);
    let read_binds_params = generate_read_binds_params(&object_definitions);

    let pk_names_types: Vec<_> = object_definitions.get_primary_key_type_exp();
    let struct_ident = object_definitions.get_struct_ident();

    return quote! {
        pub async fn get_by_id<'e>(
            #(#pk_names_types),*,
            pool: &::sqlx::PgPool,
        ) -> Result<Option<#struct_ident>, sqlx::Error>
        {
            let result = sqlx::query_as!(
                #struct_ident,
                #read_query,
                #(#read_binds_params),*
            )
            .fetch_optional(pool)
            .await?;

            Ok(result)
        }
    };
}
fn generate_read_query(object: &ObjectDefinition) -> String {
    let table_name = object.table_name.clone();

    let where_clause: String = object
        .primary_key_fields
        .iter()
        .enumerate()
        .map(|(i, FieldDefinition { column_name, .. })| {
            format!("{} = ${}", column_name.to_string(), i + 1)
        })
        .collect::<Vec<_>>()
        .join(" AND ");

    let select_clause = get_all_select_clause(object);

    format!(
        "SELECT {} FROM {} WHERE {};",
        select_clause, table_name, where_clause
    )
}

/// Generate binds parameters for `read`
fn generate_read_binds_params(object: &ObjectDefinition) -> Vec<proc_macro2::TokenStream> {
    object
        .primary_key_fields
        .iter()
        .map(|FieldDefinition { name, .. }| {
            quote! { #name, }
            // quote! { payload.#name }
        })
        .collect()
}


///UPDATE methods
/// 
/// 
pub fn update_query_method(object_definitions: &ObjectDefinition) -> proc_macro2::TokenStream {
    let update_query = generate_update_query(&object_definitions);
    let update_binds_params = generate_update_binds_params(&object_definitions);

    let pk_names_types: Vec<_> = object_definitions.get_primary_key_type_exp();
    let struct_ident = object_definitions.get_struct_ident();
    let update_dto_struct_ident = object_definitions.get_update_dto_struct_ident();

    return quote! {
        pub async fn update_by_id<'e>(
            #(#pk_names_types),*,
            payload: #update_dto_struct_ident,
            pool: &::sqlx::PgPool,
        ) -> Result<#struct_ident, sqlx::Error>
        {
            let result = sqlx::query_as!(
                #struct_ident,
                 #update_query,
                  #(#update_binds_params),*)
                .fetch_one(pool)
                .await?;

            Ok(result)
        }
    };
}

fn generate_update_query(object: &ObjectDefinition) -> String {
    // Create the SET clause
    let set_clause: Vec<String> = object
        .fields
        .iter()
        .enumerate()
        .map(|(i, FieldDefinition { column_name, .. })| {
            format!("{} = COALESCE(${}, {})", column_name, i + 1, column_name)
        })
        .collect();

    let set_clause_len = set_clause.len();

    let set_clause = set_clause.join(", ");

    // Create the WHERE clause
    let where_clause: String = object
        .primary_key_fields
        .iter()
        .enumerate()
        .map(|(i, FieldDefinition { name, .. })| format!("{} = ${}", name, i + set_clause_len + 1))
        .collect::<Vec<_>>()
        .join(" AND ");

    let select_clause = get_all_select_clause(object);

    // Combine everything into the final SQL query
    format!(
        "UPDATE {} SET {} WHERE {} RETURNING {};",
        object.table_name, set_clause, where_clause, select_clause
    )
}

/// Generate binds parameters
fn generate_update_binds_params(object: &ObjectDefinition) -> Vec<proc_macro2::TokenStream> {
    let mut binds: Vec<proc_macro2::TokenStream> = object
        .fields
        .iter()
        .map(|FieldDefinition { name, .. }| quote! { payload.#name })
        .collect();

    for bind in object
        .primary_key_fields
        .iter()
        .map(|FieldDefinition { name, .. }| quote! { #name })
        .collect::<Vec<proc_macro2::TokenStream>>()
    {
        binds.push(bind);
    }

    binds
}


/// DELETE methods
/// 
/// 
pub fn delete_query_method(object_definitions: &ObjectDefinition) -> proc_macro2::TokenStream {
    let delete_query = generate_delete_query(&object_definitions);
    let delete_binds_params = generate_delete_params(&object_definitions);

    let pk_names_types: Vec<_> = object_definitions.get_primary_key_type_exp();
    let struct_ident = object_definitions.get_struct_ident();

    return quote! {
         // Delete operation using sqlx
         pub async fn delete_by_id<'e>(
            #(#pk_names_types),*,
            pool: &::sqlx::PgPool,
        ) -> Result<#struct_ident, sqlx::Error>
        {
            let result = sqlx::query_as!(
                #struct_ident,
                #delete_query,
                #(#delete_binds_params),*
            )
            .fetch_one(pool)
            .await?;

            Ok(result)
        }
    };
}
fn generate_delete_query(object: &ObjectDefinition) -> String {
    let table_name = object.table_name.clone();

    let select_clause = get_all_select_clause(object);

    let where_clause: String = object
        .primary_key_fields
        .iter()
        .enumerate()
        .map(|(i, FieldDefinition { column_name, .. })| {
            format!("{} = ${}", column_name.to_string(), i + 1)
        })
        .collect::<Vec<_>>()
        .join(" AND ");

    format!(
        "DELETE FROM {} WHERE {} RETURNING {};",
        table_name, where_clause, select_clause
    )
}

// Generate parameters for `delete`
fn generate_delete_params(object: &ObjectDefinition) -> Vec<proc_macro2::TokenStream> {
    object
        .primary_key_fields
        .iter()
        .map(|FieldDefinition { name, .. }| {
            quote! { #name }
            // quote! { payload.#name }
        })
        .collect()
}




/// KEYSET Pagination methods
/// 
/// 
/// 
/// 
pub fn generate_keyset_pagination_methods(object: &ObjectDefinition) -> Vec<proc_macro2::TokenStream> {

    let table_name = object.table_name.clone();
    let struct_ident = object.get_struct_ident();

    let mut pagination_methods: Vec<proc_macro2::TokenStream> = vec![];

    for (pagination_name, field_pagination_infos) in object.pagination_metadata_groups.iter() {
        let method_name = Ident::new(
            &format!("paginate_dby_{}", pagination_name),
            proc_macro2::Span::call_site(),
        );
        
        let select_clause = format!("SELECT {} FROM {} WHERE ", get_all_select_clause(object), table_name);

        let (where_clause_fields, where_clause_values, orders_clause) = generate_keyset_pagination_clauses(field_pagination_infos);

        let keyset_pagination_struct_ident = object.get_keyset_pagination_dto_ident(pagination_name);

        pagination_methods.push(quote! {

            pub async fn #method_name<'a>(
                payload: #keyset_pagination_struct_ident,
                // order: String,
                // condition: String,
                pool: &::sqlx::PgPool,  // Tuple holding pagination field types dynamically
            ) -> Result<Vec<#struct_ident>, sqlx::Error> {

                let mut query_builder = sqlx::QueryBuilder::<sqlx::Postgres>::new(#select_clause);

                query_builder.push("(");
                #(#where_clause_fields)*
                query_builder.push(") ");


                query_builder.push(payload.condition);
                
                query_builder.push(" ( ");
                let mut separated = query_builder.separated(", ");
                 #(#where_clause_values)*
                separated.push_unseparated(") ");


                query_builder.push(" ORDER BY ");
                #(#orders_clause)*

                query_builder.push(" LIMIT ");
                query_builder.push_bind(payload.limit);

                let results = query_builder.build_query_as::<#struct_ident>().fetch_all(pool).await?;

                Ok(results)
            }
        });
    }

    pagination_methods
}

fn generate_keyset_pagination_clauses(
    field_pagination_infos: &Vec<PaginationMetadata>
) -> (Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>) {
    
    let mut columns = vec![];
    let mut binds = vec![];
    let mut orders = vec![];
    let mut comma = false;

    for field in field_pagination_infos.iter() {
        let field_ident = field.field_name.clone();
        let mut field_name_str = field.column_name.to_string();

        if comma {
            field_name_str = format!(", {}", field_name_str);
        }
        columns.push(quote! {
            query_builder.push(#field_name_str);
        });
        binds.push(quote! {
            separated.push_bind(payload.#field_ident);
        });
        orders.push(quote! {
            query_builder.push(#field_name_str);
            query_builder.push(" ");
            query_builder.push(payload.order_by.clone());
                
        });
        comma = true;
    }

    (columns, binds, orders)
}

/// PAGE PAGINATION methods
/// 
/// 
pub fn page_pagination_query_method(
    object_definitions: &ObjectDefinition,
) -> proc_macro2::TokenStream {
    let read_query = generate_page_pagination_query(&object_definitions);
    let read_binds_params = generate_page_pagination_binds_params(&object_definitions);

    let struct_ident = object_definitions.get_struct_ident();
    let page_pagination_struct_ident = object_definitions.get_page_pagination_struct_ident();

    return quote! {
        pub async fn get_paged<'e>(
            payload: #page_pagination_struct_ident,
            pool: &::sqlx::PgPool,
        ) -> Result<Vec<#struct_ident>, sqlx::Error>
        {
            let page_offset = (payload.page - 1) * payload.page_size;

            let result = sqlx::query_as!(
                #struct_ident,
                #read_query,
                #read_binds_params
            )
            .fetch_all(pool)
            .await?;

            Ok(result)
        }
    };
}

fn generate_page_pagination_query(object: &ObjectDefinition) -> String {
    let table_name = object.table_name.clone();

    let select_clause = get_all_select_clause(object);

    format!(
        "SELECT {} FROM {} OFFSET $1 LIMIT $2;",
        select_clause, table_name
    )
}


fn generate_page_pagination_binds_params(_object: &ObjectDefinition) -> proc_macro2::TokenStream {
    quote! { page_offset, payload.page_size}
}


///
/// 
/// FILTER
/// 
/// 

pub fn generate_filter_method(object: &ObjectDefinition) -> proc_macro2::TokenStream {

    let table_name = object.table_name.clone();

    let struct_ident = object.get_struct_ident();
    
    let select_clause = format!("SELECT {} FROM {} WHERE ", get_all_select_clause(object), table_name);

    let filter_clauses = generate_filter_clauses(object);

    let filter_struct_ident = object.get_filter_dto_struct_ident();
quote! {

    pub async fn filter<'a>(
        payload: #filter_struct_ident,
        pool: &::sqlx::PgPool,  
    ) -> Result<Vec<#struct_ident>, sqlx::Error> {

        let mut query_builder = sqlx::QueryBuilder::<sqlx::Postgres>::new(#select_clause);
        let mut and_sep = false;

        #(#filter_clauses)*

        let results = query_builder.build_query_as::<#struct_ident>().fetch_all(pool).await?;

        Ok(results)
    }
}
}

fn generate_filter_clauses(
    object: &ObjectDefinition
) -> Vec<proc_macro2::TokenStream> {
    
    let mut filter_clauses = vec![];

    for field in object.fields.iter() {
        let field_ident = field.name.clone();
        let column_name = field.column_name.to_string();
        let condition = create_ident(format!("{}_condition", field.name).as_str());

        filter_clauses.push(quote! {
            if payload.#field_ident.is_some() && payload.#condition.is_some() {
                if and_sep {
                    query_builder.push(" AND ");
                }
                
                query_builder.push(#column_name);
                query_builder.push(payload.#condition.unwrap());
                query_builder.push_bind(payload.#field_ident);
                and_sep = true;
            }
        });
    }

    filter_clauses
}

// COMMON
fn get_all_select_clause(object: &ObjectDefinition) -> String {
    object
        .fields
        .iter()
        .map(
            |FieldDefinition {
                 name, column_name, ..
             }| format!("{} AS {}", column_name, name),
        )
        .collect::<Vec<String>>()
        .join(", ")
}

