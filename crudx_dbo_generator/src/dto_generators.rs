use crate::{input::{ObjectDefinition, PaginationMetadata}, utils::create_ident};
use quote::quote;
use syn::Ident;

pub fn generate_create_dto(object: &ObjectDefinition) -> proc_macro2::TokenStream {
    let struct_ident = object.get_create_dto_struct_ident();

    let fields: Vec<_> = object
        .fields
        .iter()
        .map(|f| {
            let field_type = match f.is_nullable || f.has_default {
                true => {
                    let field_type = f.field_type.clone();
                    quote! { Option<#field_type> }
                }
                false => {
                    let field_type = f.field_type.clone();
                    quote! { #field_type }
                }
            };
            let field_name = &f.name;
            quote! { pub #field_name: #field_type, }
        })
        .collect();

    quote! {
        pub struct #struct_ident {
            #(#fields)*
        }
    }
}

// Generate the Update DTO
pub fn generate_update_dto(object: &ObjectDefinition) -> proc_macro2::TokenStream {
    let struct_ident = object.get_update_dto_struct_ident();

    let fields: Vec<_> = object
        .fields
        .iter()
        .map(|f| {
            let field_type = match f.is_primary_key {
                true => {
                    let field_type = f.field_type.clone();
                    quote! { Option<#field_type> }
                }
                false => {
                    let field_type = f.field_type.clone();
                    quote! { Option<#field_type> }
                }
            };
            let field_name = &f.name;
            quote! { pub #field_name: #field_type, }
        })
        .collect();

    quote! {
        pub struct #struct_ident {
            #(#fields)*
        }
    }
}

pub fn generate_keyset_pagination_structs(
    object: &ObjectDefinition,
) -> Vec<proc_macro2::TokenStream> {
    let mut pagination_structs: Vec<proc_macro2::TokenStream> = vec![];

    for (pagination_name, field_pagination_infos) in object.pagination_metadata_groups.iter() {
        let keyset_pagination_struct_ident =
            object.get_keyset_pagination_dto_ident(pagination_name);

        pagination_structs.push(generate_keyset_pagination_dto(
            keyset_pagination_struct_ident.clone(),
            field_pagination_infos,
        ));
    }

    pagination_structs
}

pub fn generate_keyset_pagination_dto(
    keyset_pagination_struct_ident: Ident,
    field_pagination_infos: &Vec<PaginationMetadata>,
) -> proc_macro2::TokenStream {
    let fields: Vec<_> = field_pagination_infos
        .iter()
        .map(
            |PaginationMetadata {
                 field_name,
                 field_type,
                 ..
             }| {
                quote! { pub #field_name: #field_type, }
            },
        )
        .collect();
    quote! {
        pub struct #keyset_pagination_struct_ident {
            #(#fields)*
            pub condition: String,
            pub order_by: String,
            pub limit: i64,
        }
    }
}

pub fn generate_page_pagination_dto(object: &ObjectDefinition) -> proc_macro2::TokenStream {
    let page_pagination_struct_ident = object.get_page_pagination_struct_ident();

    quote! {
        pub struct #page_pagination_struct_ident {
            pub page: i64,
            pub page_size: i64,
        }
    }
}

pub fn generate_filter_dto(object: &ObjectDefinition) -> proc_macro2::TokenStream {
    let struct_ident = object.get_filter_dto_struct_ident();

    let fields: Vec<_> = object
        .fields
        .iter()
        .map(|f| {
            let field_type = &f.field_type;
            let field_name = &f.name;
            quote! { pub #field_name: Option<#field_type>, }
        })
        .collect();

    let conditions: Vec<_> = object
        .fields
        .iter()
        .map(|f| {
            let field_name =  create_ident(format!("{}_condition",&f.name).as_str());
            quote! { pub #field_name: Option<String>, }
        })
        .collect();

    quote! {

        #[derive(Default)]
        pub struct #struct_ident {
            #(#fields)*
            #(#conditions)*
        }
    }
}
