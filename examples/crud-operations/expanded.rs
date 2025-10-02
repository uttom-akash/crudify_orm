"#[derive(Default)] pub struct PartnerDBOFilter\n{\n    pub id : Option < i64 > , pub name : Option < String > , pub partner_type\n    : Option < String > , pub created_at : Option < NaiveDateTime > , pub\n    enabled : Option < bool > , pub id_condition : Option < String > , pub\n    name_condition : Option < String > , pub partner_type_condition : Option <\n    String > , pub created_at_condition : Option < String > , pub\n    enabled_condition : Option < String > ,\n}"
#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2021::*;
use chrono::{Duration, NaiveDateTime, Utc};
use crudify_orm::Entity;
use rand::Rng;
use sqlx::{FromRow, PgPool};
#[entity(table_name = "partners")]
struct PartnerDBO {
    #[entity(id, keyset_pagination("id_created_at"))]
    id: i64,
    #[entity(alias = "partner_name")]
    name: String,
    partner_type: String,
    #[entity(default, keyset_pagination("id_created_at"))]
    created_at: NaiveDateTime,
    enabled: bool,
}
#[automatically_derived]
impl ::core::fmt::Debug for PartnerDBO {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field5_finish(
            f,
            "PartnerDBO",
            "id",
            &self.id,
            "name",
            &self.name,
            "partner_type",
            &self.partner_type,
            "created_at",
            &self.created_at,
            "enabled",
            &&self.enabled,
        )
    }
}
pub struct PartnerDBOCreate {
    pub id: i64,
    pub name: String,
    pub partner_type: String,
    pub created_at: Option<NaiveDateTime>,
    pub enabled: bool,
}
pub struct PartnerDBOUpdate {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub partner_type: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub enabled: Option<bool>,
}
pub struct PagePagination {
    pub page: i64,
    pub page_size: i64,
}
pub struct PaginationCursorIdCreatedAt {
    pub id: i64,
    pub created_at: NaiveDateTime,
    pub condition: String,
    pub order_by: String,
    pub limit: i64,
}
pub struct PartnerDBOFilter {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub partner_type: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub enabled: Option<bool>,
    pub id_condition: Option<String>,
    pub name_condition: Option<String>,
    pub partner_type_condition: Option<String>,
    pub created_at_condition: Option<String>,
    pub enabled_condition: Option<String>,
}
#[automatically_derived]
impl ::core::default::Default for PartnerDBOFilter {
    #[inline]
    fn default() -> PartnerDBOFilter {
        PartnerDBOFilter {
            id: ::core::default::Default::default(),
            name: ::core::default::Default::default(),
            partner_type: ::core::default::Default::default(),
            created_at: ::core::default::Default::default(),
            enabled: ::core::default::Default::default(),
            id_condition: ::core::default::Default::default(),
            name_condition: ::core::default::Default::default(),
            partner_type_condition: ::core::default::Default::default(),
            created_at_condition: ::core::default::Default::default(),
            enabled_condition: ::core::default::Default::default(),
        }
    }
}
impl PartnerDBO {
    pub async fn create<'a>(
        payload: PartnerDBOCreate,
        pool: &::sqlx::PgPool,
    ) -> Result<PartnerDBO, sqlx::Error> {
        let mut query_builder = sqlx::QueryBuilder::<
            sqlx::Postgres,
        >::new("INSERT INTO partners (");
        query_builder.push("id");
        query_builder.push(", partner_name");
        query_builder.push(", partner_type");
        if payload.created_at.is_some() {
            query_builder.push(", created_at");
        }
        query_builder.push(", enabled");
        query_builder.push(") VALUES (");
        let mut separated = query_builder.separated(", ");
        separated.push_bind(payload.id);
        separated.push_bind(payload.name);
        separated.push_bind(payload.partner_type);
        if payload.created_at.is_some() {
            separated.push_bind(payload.created_at);
        }
        separated.push_bind(payload.enabled);
        separated.push_unseparated(") ");
        query_builder
            .push(
                " RETURNING id AS id, partner_name AS name, partner_type AS partner_type, created_at AS created_at, enabled AS enabled",
            );
        let result = query_builder.build_query_as::<PartnerDBO>().fetch_one(pool).await?;
        Ok(result)
    }
    pub async fn get_by_id<'e>(
        id: i64,
        pool: &::sqlx::PgPool,
    ) -> Result<Option<PartnerDBO>, sqlx::Error> {
        let result = {
            {
                #[allow(clippy::all)]
                {
                    use ::sqlx::Arguments as _;
                    let arg0 = &(id);
                    if false {
                        use ::sqlx::ty_match::{WrapSameExt as _, MatchBorrowExt as _};
                        let expr = ::sqlx::ty_match::dupe_value(arg0);
                        let ty_check = ::sqlx::ty_match::WrapSame::<i64, _>::new(&expr)
                            .wrap_same();
                        let (mut _ty_check, match_borrow) = ::sqlx::ty_match::MatchBorrow::new(
                            ty_check,
                            &expr,
                        );
                        _ty_check = match_borrow.match_borrow();
                        {
                            #[cold]
                            #[track_caller]
                            #[inline(never)]
                            const fn panic_cold_explicit() -> ! {
                                ::core::panicking::panic_explicit()
                            }
                            panic_cold_explicit();
                        };
                    }
                    let mut query_args = <sqlx::postgres::Postgres as ::sqlx::database::Database>::Arguments::<
                        '_,
                    >::default();
                    query_args
                        .reserve(
                            1usize,
                            0
                                + ::sqlx::encode::Encode::<
                                    sqlx::postgres::Postgres,
                                >::size_hint(arg0),
                        );
                    let query_args = ::core::result::Result::<
                        _,
                        ::sqlx::error::BoxDynError,
                    >::Ok(query_args)
                        .and_then(move |mut query_args| {
                            query_args.add(arg0).map(move |()| query_args)
                        });
                    ::sqlx::__query_with_result::<
                        sqlx::postgres::Postgres,
                        _,
                    >(
                            "SELECT id AS id, partner_name AS name, partner_type AS partner_type, created_at AS created_at, enabled AS enabled FROM partners WHERE id = $1;",
                            query_args,
                        )
                        .try_map(|row: sqlx::postgres::PgRow| {
                            use ::sqlx::Row as _;
                            #[allow(non_snake_case)]
                            let sqlx_query_as_id = row
                                .try_get_unchecked::<i64, _>(0usize)?
                                .into();
                            #[allow(non_snake_case)]
                            let sqlx_query_as_name = row
                                .try_get_unchecked::<String, _>(1usize)?
                                .into();
                            #[allow(non_snake_case)]
                            let sqlx_query_as_partner_type = row
                                .try_get_unchecked::<String, _>(2usize)?
                                .into();
                            #[allow(non_snake_case)]
                            let sqlx_query_as_created_at = row
                                .try_get_unchecked::<
                                    sqlx::types::chrono::NaiveDateTime,
                                    _,
                                >(3usize)?
                                .into();
                            #[allow(non_snake_case)]
                            let sqlx_query_as_enabled = row
                                .try_get_unchecked::<bool, _>(4usize)?
                                .into();
                            ::std::result::Result::Ok(PartnerDBO {
                                id: sqlx_query_as_id,
                                name: sqlx_query_as_name,
                                partner_type: sqlx_query_as_partner_type,
                                created_at: sqlx_query_as_created_at,
                                enabled: sqlx_query_as_enabled,
                            })
                        })
                }
            }
        }
            .fetch_optional(pool)
            .await?;
        Ok(result)
    }
    pub async fn update_by_id<'e>(
        id: i64,
        payload: PartnerDBOUpdate,
        pool: &::sqlx::PgPool,
    ) -> Result<PartnerDBO, sqlx::Error> {
        let result = {
            {
                #[allow(clippy::all)]
                {
                    use ::sqlx::Arguments as _;
                    let arg0 = &(payload.id);
                    let arg1 = &(payload.name);
                    let arg2 = &(payload.partner_type);
                    let arg3 = &(payload.created_at);
                    let arg4 = &(payload.enabled);
                    let arg5 = &(id);
                    if false {
                        use ::sqlx::ty_match::{WrapSameExt as _, MatchBorrowExt as _};
                        let expr = ::sqlx::ty_match::dupe_value(arg0);
                        let ty_check = ::sqlx::ty_match::WrapSame::<i64, _>::new(&expr)
                            .wrap_same();
                        let (mut _ty_check, match_borrow) = ::sqlx::ty_match::MatchBorrow::new(
                            ty_check,
                            &expr,
                        );
                        _ty_check = match_borrow.match_borrow();
                        {
                            #[cold]
                            #[track_caller]
                            #[inline(never)]
                            const fn panic_cold_explicit() -> ! {
                                ::core::panicking::panic_explicit()
                            }
                            panic_cold_explicit();
                        };
                    }
                    if false {
                        use ::sqlx::ty_match::{WrapSameExt as _, MatchBorrowExt as _};
                        let expr = ::sqlx::ty_match::dupe_value(arg1);
                        let ty_check = ::sqlx::ty_match::WrapSame::<&str, _>::new(&expr)
                            .wrap_same();
                        let (mut _ty_check, match_borrow) = ::sqlx::ty_match::MatchBorrow::new(
                            ty_check,
                            &expr,
                        );
                        _ty_check = match_borrow.match_borrow();
                        {
                            #[cold]
                            #[track_caller]
                            #[inline(never)]
                            const fn panic_cold_explicit() -> ! {
                                ::core::panicking::panic_explicit()
                            }
                            panic_cold_explicit();
                        };
                    }
                    if false {
                        use ::sqlx::ty_match::{WrapSameExt as _, MatchBorrowExt as _};
                        let expr = ::sqlx::ty_match::dupe_value(arg2);
                        let ty_check = ::sqlx::ty_match::WrapSame::<&str, _>::new(&expr)
                            .wrap_same();
                        let (mut _ty_check, match_borrow) = ::sqlx::ty_match::MatchBorrow::new(
                            ty_check,
                            &expr,
                        );
                        _ty_check = match_borrow.match_borrow();
                        {
                            #[cold]
                            #[track_caller]
                            #[inline(never)]
                            const fn panic_cold_explicit() -> ! {
                                ::core::panicking::panic_explicit()
                            }
                            panic_cold_explicit();
                        };
                    }
                    if false {
                        use ::sqlx::ty_match::{WrapSameExt as _, MatchBorrowExt as _};
                        let expr = ::sqlx::ty_match::dupe_value(arg3);
                        let ty_check = ::sqlx::ty_match::WrapSame::<
                            sqlx::types::chrono::NaiveDateTime,
                            _,
                        >::new(&expr)
                            .wrap_same();
                        let (mut _ty_check, match_borrow) = ::sqlx::ty_match::MatchBorrow::new(
                            ty_check,
                            &expr,
                        );
                        _ty_check = match_borrow.match_borrow();
                        {
                            #[cold]
                            #[track_caller]
                            #[inline(never)]
                            const fn panic_cold_explicit() -> ! {
                                ::core::panicking::panic_explicit()
                            }
                            panic_cold_explicit();
                        };
                    }
                    if false {
                        use ::sqlx::ty_match::{WrapSameExt as _, MatchBorrowExt as _};
                        let expr = ::sqlx::ty_match::dupe_value(arg4);
                        let ty_check = ::sqlx::ty_match::WrapSame::<bool, _>::new(&expr)
                            .wrap_same();
                        let (mut _ty_check, match_borrow) = ::sqlx::ty_match::MatchBorrow::new(
                            ty_check,
                            &expr,
                        );
                        _ty_check = match_borrow.match_borrow();
                        {
                            #[cold]
                            #[track_caller]
                            #[inline(never)]
                            const fn panic_cold_explicit() -> ! {
                                ::core::panicking::panic_explicit()
                            }
                            panic_cold_explicit();
                        };
                    }
                    if false {
                        use ::sqlx::ty_match::{WrapSameExt as _, MatchBorrowExt as _};
                        let expr = ::sqlx::ty_match::dupe_value(arg5);
                        let ty_check = ::sqlx::ty_match::WrapSame::<i64, _>::new(&expr)
                            .wrap_same();
                        let (mut _ty_check, match_borrow) = ::sqlx::ty_match::MatchBorrow::new(
                            ty_check,
                            &expr,
                        );
                        _ty_check = match_borrow.match_borrow();
                        {
                            #[cold]
                            #[track_caller]
                            #[inline(never)]
                            const fn panic_cold_explicit() -> ! {
                                ::core::panicking::panic_explicit()
                            }
                            panic_cold_explicit();
                        };
                    }
                    let mut query_args = <sqlx::postgres::Postgres as ::sqlx::database::Database>::Arguments::<
                        '_,
                    >::default();
                    query_args
                        .reserve(
                            6usize,
                            0
                                + ::sqlx::encode::Encode::<
                                    sqlx::postgres::Postgres,
                                >::size_hint(arg0)
                                + ::sqlx::encode::Encode::<
                                    sqlx::postgres::Postgres,
                                >::size_hint(arg1)
                                + ::sqlx::encode::Encode::<
                                    sqlx::postgres::Postgres,
                                >::size_hint(arg2)
                                + ::sqlx::encode::Encode::<
                                    sqlx::postgres::Postgres,
                                >::size_hint(arg3)
                                + ::sqlx::encode::Encode::<
                                    sqlx::postgres::Postgres,
                                >::size_hint(arg4)
                                + ::sqlx::encode::Encode::<
                                    sqlx::postgres::Postgres,
                                >::size_hint(arg5),
                        );
                    let query_args = ::core::result::Result::<
                        _,
                        ::sqlx::error::BoxDynError,
                    >::Ok(query_args)
                        .and_then(move |mut query_args| {
                            query_args.add(arg0).map(move |()| query_args)
                        })
                        .and_then(move |mut query_args| {
                            query_args.add(arg1).map(move |()| query_args)
                        })
                        .and_then(move |mut query_args| {
                            query_args.add(arg2).map(move |()| query_args)
                        })
                        .and_then(move |mut query_args| {
                            query_args.add(arg3).map(move |()| query_args)
                        })
                        .and_then(move |mut query_args| {
                            query_args.add(arg4).map(move |()| query_args)
                        })
                        .and_then(move |mut query_args| {
                            query_args.add(arg5).map(move |()| query_args)
                        });
                    ::sqlx::__query_with_result::<
                        sqlx::postgres::Postgres,
                        _,
                    >(
                            "UPDATE partners SET id = COALESCE($1, id), partner_name = COALESCE($2, partner_name), partner_type = COALESCE($3, partner_type), created_at = COALESCE($4, created_at), enabled = COALESCE($5, enabled) WHERE id = $6 RETURNING id AS id, partner_name AS name, partner_type AS partner_type, created_at AS created_at, enabled AS enabled;",
                            query_args,
                        )
                        .try_map(|row: sqlx::postgres::PgRow| {
                            use ::sqlx::Row as _;
                            #[allow(non_snake_case)]
                            let sqlx_query_as_id = row
                                .try_get_unchecked::<i64, _>(0usize)?
                                .into();
                            #[allow(non_snake_case)]
                            let sqlx_query_as_name = row
                                .try_get_unchecked::<String, _>(1usize)?
                                .into();
                            #[allow(non_snake_case)]
                            let sqlx_query_as_partner_type = row
                                .try_get_unchecked::<String, _>(2usize)?
                                .into();
                            #[allow(non_snake_case)]
                            let sqlx_query_as_created_at = row
                                .try_get_unchecked::<
                                    sqlx::types::chrono::NaiveDateTime,
                                    _,
                                >(3usize)?
                                .into();
                            #[allow(non_snake_case)]
                            let sqlx_query_as_enabled = row
                                .try_get_unchecked::<bool, _>(4usize)?
                                .into();
                            ::std::result::Result::Ok(PartnerDBO {
                                id: sqlx_query_as_id,
                                name: sqlx_query_as_name,
                                partner_type: sqlx_query_as_partner_type,
                                created_at: sqlx_query_as_created_at,
                                enabled: sqlx_query_as_enabled,
                            })
                        })
                }
            }
        }
            .fetch_one(pool)
            .await?;
        Ok(result)
    }
    pub async fn delete_by_id<'e>(
        id: i64,
        pool: &::sqlx::PgPool,
    ) -> Result<PartnerDBO, sqlx::Error> {
        let result = {
            {
                #[allow(clippy::all)]
                {
                    use ::sqlx::Arguments as _;
                    let arg0 = &(id);
                    if false {
                        use ::sqlx::ty_match::{WrapSameExt as _, MatchBorrowExt as _};
                        let expr = ::sqlx::ty_match::dupe_value(arg0);
                        let ty_check = ::sqlx::ty_match::WrapSame::<i64, _>::new(&expr)
                            .wrap_same();
                        let (mut _ty_check, match_borrow) = ::sqlx::ty_match::MatchBorrow::new(
                            ty_check,
                            &expr,
                        );
                        _ty_check = match_borrow.match_borrow();
                        {
                            #[cold]
                            #[track_caller]
                            #[inline(never)]
                            const fn panic_cold_explicit() -> ! {
                                ::core::panicking::panic_explicit()
                            }
                            panic_cold_explicit();
                        };
                    }
                    let mut query_args = <sqlx::postgres::Postgres as ::sqlx::database::Database>::Arguments::<
                        '_,
                    >::default();
                    query_args
                        .reserve(
                            1usize,
                            0
                                + ::sqlx::encode::Encode::<
                                    sqlx::postgres::Postgres,
                                >::size_hint(arg0),
                        );
                    let query_args = ::core::result::Result::<
                        _,
                        ::sqlx::error::BoxDynError,
                    >::Ok(query_args)
                        .and_then(move |mut query_args| {
                            query_args.add(arg0).map(move |()| query_args)
                        });
                    ::sqlx::__query_with_result::<
                        sqlx::postgres::Postgres,
                        _,
                    >(
                            "DELETE FROM partners WHERE id = $1 RETURNING id AS id, partner_name AS name, partner_type AS partner_type, created_at AS created_at, enabled AS enabled;",
                            query_args,
                        )
                        .try_map(|row: sqlx::postgres::PgRow| {
                            use ::sqlx::Row as _;
                            #[allow(non_snake_case)]
                            let sqlx_query_as_id = row
                                .try_get_unchecked::<i64, _>(0usize)?
                                .into();
                            #[allow(non_snake_case)]
                            let sqlx_query_as_name = row
                                .try_get_unchecked::<String, _>(1usize)?
                                .into();
                            #[allow(non_snake_case)]
                            let sqlx_query_as_partner_type = row
                                .try_get_unchecked::<String, _>(2usize)?
                                .into();
                            #[allow(non_snake_case)]
                            let sqlx_query_as_created_at = row
                                .try_get_unchecked::<
                                    sqlx::types::chrono::NaiveDateTime,
                                    _,
                                >(3usize)?
                                .into();
                            #[allow(non_snake_case)]
                            let sqlx_query_as_enabled = row
                                .try_get_unchecked::<bool, _>(4usize)?
                                .into();
                            ::std::result::Result::Ok(PartnerDBO {
                                id: sqlx_query_as_id,
                                name: sqlx_query_as_name,
                                partner_type: sqlx_query_as_partner_type,
                                created_at: sqlx_query_as_created_at,
                                enabled: sqlx_query_as_enabled,
                            })
                        })
                }
            }
        }
            .fetch_one(pool)
            .await?;
        Ok(result)
    }
    pub async fn paginate_dby_id_created_at<'a>(
        payload: PaginationCursorIdCreatedAt,
        pool: &::sqlx::PgPool,
    ) -> Result<Vec<PartnerDBO>, sqlx::Error> {
        let mut query_builder = sqlx::QueryBuilder::<
            sqlx::Postgres,
        >::new(
            "SELECT id AS id, partner_name AS name, partner_type AS partner_type, created_at AS created_at, enabled AS enabled FROM partners WHERE ",
        );
        query_builder.push("(");
        query_builder.push("id");
        query_builder.push(", created_at");
        query_builder.push(") ");
        query_builder.push(payload.condition);
        query_builder.push(" ( ");
        let mut separated = query_builder.separated(", ");
        separated.push_bind(payload.id);
        separated.push_bind(payload.created_at);
        separated.push_unseparated(") ");
        query_builder.push(" ORDER BY ");
        query_builder.push("id");
        query_builder.push(" ");
        query_builder.push(payload.order_by.clone());
        query_builder.push(", created_at");
        query_builder.push(" ");
        query_builder.push(payload.order_by.clone());
        query_builder.push(" LIMIT ");
        query_builder.push_bind(payload.limit);
        let results = query_builder
            .build_query_as::<PartnerDBO>()
            .fetch_all(pool)
            .await?;
        Ok(results)
    }
    pub async fn get_paged<'e>(
        payload: PagePagination,
        pool: &::sqlx::PgPool,
    ) -> Result<Vec<PartnerDBO>, sqlx::Error> {
        let page_offset = (payload.page - 1) * payload.page_size;
        let result = {
            {
                #[allow(clippy::all)]
                {
                    use ::sqlx::Arguments as _;
                    let arg0 = &(page_offset);
                    let arg1 = &(payload.page_size);
                    if false {
                        use ::sqlx::ty_match::{WrapSameExt as _, MatchBorrowExt as _};
                        let expr = ::sqlx::ty_match::dupe_value(arg0);
                        let ty_check = ::sqlx::ty_match::WrapSame::<i64, _>::new(&expr)
                            .wrap_same();
                        let (mut _ty_check, match_borrow) = ::sqlx::ty_match::MatchBorrow::new(
                            ty_check,
                            &expr,
                        );
                        _ty_check = match_borrow.match_borrow();
                        {
                            #[cold]
                            #[track_caller]
                            #[inline(never)]
                            const fn panic_cold_explicit() -> ! {
                                ::core::panicking::panic_explicit()
                            }
                            panic_cold_explicit();
                        };
                    }
                    if false {
                        use ::sqlx::ty_match::{WrapSameExt as _, MatchBorrowExt as _};
                        let expr = ::sqlx::ty_match::dupe_value(arg1);
                        let ty_check = ::sqlx::ty_match::WrapSame::<i64, _>::new(&expr)
                            .wrap_same();
                        let (mut _ty_check, match_borrow) = ::sqlx::ty_match::MatchBorrow::new(
                            ty_check,
                            &expr,
                        );
                        _ty_check = match_borrow.match_borrow();
                        {
                            #[cold]
                            #[track_caller]
                            #[inline(never)]
                            const fn panic_cold_explicit() -> ! {
                                ::core::panicking::panic_explicit()
                            }
                            panic_cold_explicit();
                        };
                    }
                    let mut query_args = <sqlx::postgres::Postgres as ::sqlx::database::Database>::Arguments::<
                        '_,
                    >::default();
                    query_args
                        .reserve(
                            2usize,
                            0
                                + ::sqlx::encode::Encode::<
                                    sqlx::postgres::Postgres,
                                >::size_hint(arg0)
                                + ::sqlx::encode::Encode::<
                                    sqlx::postgres::Postgres,
                                >::size_hint(arg1),
                        );
                    let query_args = ::core::result::Result::<
                        _,
                        ::sqlx::error::BoxDynError,
                    >::Ok(query_args)
                        .and_then(move |mut query_args| {
                            query_args.add(arg0).map(move |()| query_args)
                        })
                        .and_then(move |mut query_args| {
                            query_args.add(arg1).map(move |()| query_args)
                        });
                    ::sqlx::__query_with_result::<
                        sqlx::postgres::Postgres,
                        _,
                    >(
                            "SELECT id AS id, partner_name AS name, partner_type AS partner_type, created_at AS created_at, enabled AS enabled FROM partners OFFSET $1 LIMIT $2;",
                            query_args,
                        )
                        .try_map(|row: sqlx::postgres::PgRow| {
                            use ::sqlx::Row as _;
                            #[allow(non_snake_case)]
                            let sqlx_query_as_id = row
                                .try_get_unchecked::<i64, _>(0usize)?
                                .into();
                            #[allow(non_snake_case)]
                            let sqlx_query_as_name = row
                                .try_get_unchecked::<String, _>(1usize)?
                                .into();
                            #[allow(non_snake_case)]
                            let sqlx_query_as_partner_type = row
                                .try_get_unchecked::<String, _>(2usize)?
                                .into();
                            #[allow(non_snake_case)]
                            let sqlx_query_as_created_at = row
                                .try_get_unchecked::<
                                    sqlx::types::chrono::NaiveDateTime,
                                    _,
                                >(3usize)?
                                .into();
                            #[allow(non_snake_case)]
                            let sqlx_query_as_enabled = row
                                .try_get_unchecked::<bool, _>(4usize)?
                                .into();
                            ::std::result::Result::Ok(PartnerDBO {
                                id: sqlx_query_as_id,
                                name: sqlx_query_as_name,
                                partner_type: sqlx_query_as_partner_type,
                                created_at: sqlx_query_as_created_at,
                                enabled: sqlx_query_as_enabled,
                            })
                        })
                }
            }
        }
            .fetch_all(pool)
            .await?;
        Ok(result)
    }
    pub async fn filter<'a>(
        payload: PartnerDBOFilter,
        pool: &::sqlx::PgPool,
    ) -> Result<Vec<PartnerDBO>, sqlx::Error> {
        let mut query_builder = sqlx::QueryBuilder::<
            sqlx::Postgres,
        >::new(
            "SELECT id AS id, partner_name AS name, partner_type AS partner_type, created_at AS created_at, enabled AS enabled FROM partners WHERE ",
        );
        let mut and_sep = false;
        if payload.id.is_some() && payload.id_condition.is_some() {
            if and_sep {
                query_builder.push(" AND");
            }
            query_builder.push("id");
            query_builder.push(payload.id_condition.unwrap());
            query_builder.push_bind(payload.id);
            and_sep = true;
        }
        if payload.name.is_some() && payload.name_condition.is_some() {
            if and_sep {
                query_builder.push(" AND");
            }
            query_builder.push("partner_name");
            query_builder.push(payload.name_condition.unwrap());
            query_builder.push_bind(payload.name);
            and_sep = true;
        }
        if payload.partner_type.is_some() && payload.partner_type_condition.is_some() {
            if and_sep {
                query_builder.push(" AND");
            }
            query_builder.push("partner_type");
            query_builder.push(payload.partner_type_condition.unwrap());
            query_builder.push_bind(payload.partner_type);
            and_sep = true;
        }
        if payload.created_at.is_some() && payload.created_at_condition.is_some() {
            if and_sep {
                query_builder.push(" AND");
            }
            query_builder.push("created_at");
            query_builder.push(payload.created_at_condition.unwrap());
            query_builder.push_bind(payload.created_at);
            and_sep = true;
        }
        if payload.enabled.is_some() && payload.enabled_condition.is_some() {
            if and_sep {
                query_builder.push(" AND");
            }
            query_builder.push("enabled");
            query_builder.push(payload.enabled_condition.unwrap());
            query_builder.push_bind(payload.enabled);
            and_sep = true;
        }
        let results = query_builder
            .build_query_as::<PartnerDBO>()
            .fetch_all(pool)
            .await?;
        Ok(results)
    }
}
#[automatically_derived]
impl<'a, R: ::sqlx::Row> ::sqlx::FromRow<'a, R> for PartnerDBO
where
    &'a ::std::primitive::str: ::sqlx::ColumnIndex<R>,
    i64: ::sqlx::decode::Decode<'a, R::Database>,
    i64: ::sqlx::types::Type<R::Database>,
    String: ::sqlx::decode::Decode<'a, R::Database>,
    String: ::sqlx::types::Type<R::Database>,
    String: ::sqlx::decode::Decode<'a, R::Database>,
    String: ::sqlx::types::Type<R::Database>,
    NaiveDateTime: ::sqlx::decode::Decode<'a, R::Database>,
    NaiveDateTime: ::sqlx::types::Type<R::Database>,
    bool: ::sqlx::decode::Decode<'a, R::Database>,
    bool: ::sqlx::types::Type<R::Database>,
{
    fn from_row(__row: &'a R) -> ::sqlx::Result<Self> {
        let id: i64 = __row.try_get("id")?;
        let name: String = __row.try_get("name")?;
        let partner_type: String = __row.try_get("partner_type")?;
        let created_at: NaiveDateTime = __row.try_get("created_at")?;
        let enabled: bool = __row.try_get("enabled")?;
        ::std::result::Result::Ok(PartnerDBO {
            id,
            name,
            partner_type,
            created_at,
            enabled,
        })
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body = async {
        let database_url = "postgres://crudx:crudx@127.0.0.1:5432/test_pagination_sqlx";
        let pool = PgPool::connect(database_url).await?;
        let id: i64 = rand::thread_rng().gen_range(1..=10000);
        let updated_id: i64 = rand::thread_rng().gen_range(10000..=100000);
        let partner_create_dto = PartnerDBOCreate {
            id: id.clone(),
            name: ::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("akash-test partner-{0}", id))
            }),
            partner_type: "test".to_string(),
            created_at: None,
            enabled: true,
        };
        let created_partner = PartnerDBO::create(partner_create_dto, &pool).await?;
        {
            ::std::io::_print(
                format_args!(
                    "########### created partner: {0:?} ###########\n",
                    created_partner,
                ),
            );
        };
        let partner_update_dto = PartnerDBOUpdate {
            id: Some(updated_id.clone()),
            name: Some(
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(format_args!("updated partner-{0}", updated_id))
                }),
            ),
            partner_type: None,
            created_at: None,
            enabled: Some(false),
        };
        let updated_partner = PartnerDBO::update_by_id(
                created_partner.id,
                partner_update_dto,
                &pool,
            )
            .await?;
        {
            ::std::io::_print(
                format_args!(
                    "########### updated partner: {0:?} ###########\n",
                    updated_partner,
                ),
            );
        };
        let queried_partner = PartnerDBO::get_by_id(updated_partner.id, &pool).await?;
        {
            ::std::io::_print(
                format_args!(
                    "########### queried partner: {0:?} ###########\n",
                    queried_partner,
                ),
            );
        };
        {
            ::std::io::_print(
                format_args!("########### Pagination Results: ###########\n"),
            );
        };
        let now = Utc::now().naive_utc();
        let mut created_at = now - Duration::hours(12);
        let mut id: i64 = i64::MAX;
        let limit: i64 = 6;
        loop {
            let results: Vec<PartnerDBO> = PartnerDBO::paginate_dby_id_created_at(
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
            {
                ::std::io::_print(format_args!("### {0:?} : {1:?} ####\n", id, results));
            };
            match results.last() {
                Some(partner_dbo) => {
                    id = partner_dbo.id;
                    created_at = partner_dbo.created_at;
                }
                None => break,
            };
        }
        {
            ::std::io::_print(
                format_args!("########### Page Pagination Results: ###########\n"),
            );
        };
        let results: Vec<PartnerDBO> = PartnerDBO::get_paged(
                PagePagination {
                    page: 1,
                    page_size: 6,
                },
                &pool,
            )
            .await?;
        {
            ::std::io::_print(format_args!("### {0:?} ####\n", results.last()));
        };
        {
            ::std::io::_print(format_args!("### ---------------- ####\n"));
        };
        {
            ::std::io::_print(format_args!("#### FILTER ####\n"));
        };
        let mut filter = PartnerDBOFilter::default();
        filter.id = Some(80);
        filter.id_condition = Some("<=".to_string());
        filter.enabled = Some(false);
        filter.enabled_condition = Some("=".to_string());
        let results = PartnerDBO::filter(filter, &pool).await?;
        {
            ::std::io::_print(format_args!("###### {0:?} #####\n", results));
        };
        match PartnerDBO::delete_by_id(updated_partner.id, &pool).await {
            Ok(_) => {
                ::std::io::_print(
                    format_args!("########### Good to go: partner deleted ###########\n"),
                );
            }
            Err(e) => {
                ::std::io::_print(
                    format_args!(
                        "########### ERROR: deleting partner: {0} ###########\n",
                        e,
                    ),
                );
            }
        }
        match PartnerDBO::get_by_id(updated_partner.id, &pool).await {
            Ok(Some(_)) => {
                ::std::io::_print(
                    format_args!("########### ERROR: partner found ###########\n"),
                );
            }
            _ => {
                ::std::io::_print(
                    format_args!(
                        "########### Good to go: partner should not be found ###########\n",
                    ),
                );
            }
        }
        Ok(())
    };
    #[allow(clippy::expect_used, clippy::diverging_sub_expression)]
    {
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
