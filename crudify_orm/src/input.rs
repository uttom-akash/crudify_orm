use std::collections::HashMap;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, Attribute, Ident, ItemStruct, LitStr, Meta, MetaList, Type};

use crate::{
    constant::{
        COLUMN_ALIAS_ATTRIBUTE, ENTITY_ATTRIBUTE, DEFAULT_COLUMN_ATTRIBUTE, ID_ATTRIBUTE,
        KEYSET_PAGINATION_ATTRIBUTE, TABLE_NAME_ATTRIBUTE,
    },
    utils::{create_ident, to_pascal_case},
};

#[derive(Default)]
pub struct DboAttributeMetadata {
    pub table_name: Option<String>,
}

impl DboAttributeMetadata {
    pub fn extract(parsed_struct: &ItemStruct) -> Self {
        match parsed_struct
            .attrs
            .iter()
            .find(|attr| attr.path().is_ident(ENTITY_ATTRIBUTE))
        {
            Some(Attribute {
                meta: Meta::List(MetaList { tokens, .. }),
                ..
            }) => match syn::parse2::<DboAttributeMetadata>(tokens.clone()) {
                Ok(dbo_metadata) => dbo_metadata,
                Err(_) => DboAttributeMetadata::default(),
            },
            _ => DboAttributeMetadata::default(),
        }
    }
}

impl Parse for DboAttributeMetadata {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut dbo_attr_metadata = DboAttributeMetadata::default();
        let mut expect_comma = false;

        while !input.is_empty() {
            if expect_comma {
                let _ = input.parse::<syn::token::Comma>()?;
            }

            let key = input.parse::<syn::Ident>()?.to_string();
            input.parse::<syn::token::Eq>()?;

            match key.as_str() {
                TABLE_NAME_ATTRIBUTE => {
                    dbo_attr_metadata.table_name =
                        input.parse::<LitStr>().map(|name| name.value()).ok();
                }
                _ => (),
            }
            expect_comma = true;
        }

        Ok(dbo_attr_metadata)
    }
}

#[derive(Clone, Debug)]
pub struct FieldDefinition {
    pub name: Ident,
    pub column_name: Ident,
    pub field_type: Type,
    pub is_primary_key: bool,
    pub is_nullable: bool,
    pub has_default: bool,
    pub keyset_pagination: Option<PaginationMetadata>,
}

pub struct FieldAttribute {
    pub is_primary_key: bool,
    pub has_default: bool,
    pub alias: Option<String>,
    pub keyset_pagination: Option<PaginatedByAttrData>,
}

impl Parse for FieldAttribute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut is_primary_key = false;
        let mut has_default = false;
        let mut alias: Option<String> = None;
        let mut keyset_pagination: Option<PaginatedByAttrData> = None;
        let mut expect_comma = false;

        while !input.is_empty() {
            if expect_comma {
                let _ = input.parse::<syn::token::Comma>()?;
            }

            let key = input.parse::<syn::Ident>()?.to_string();

            match key.as_str() {
                ID_ATTRIBUTE => {
                    is_primary_key = true;
                }
                DEFAULT_COLUMN_ATTRIBUTE => {
                    has_default = true;
                }
                COLUMN_ALIAS_ATTRIBUTE => {
                    input.parse::<syn::token::Eq>()?;
                    alias = input.parse::<LitStr>().map(|name| name.value()).ok();
                }
                KEYSET_PAGINATION_ATTRIBUTE => {
                    if input.peek(syn::token::Paren) {
                        let content;
                        syn::parenthesized!(content in input);
                        let group = content.parse::<LitStr>()?.value();

                        keyset_pagination = Some(PaginatedByAttrData::new(group));
                    } else {
                        keyset_pagination = Some(PaginatedByAttrData::default());
                    }
                }
                _ => (),
            }
            expect_comma = true;
        }

        Ok(FieldAttribute {
            is_primary_key,
            has_default,
            alias,
            keyset_pagination,
        })
    }
}

impl FieldDefinition {
    fn new(field: &syn::Field) -> FieldDefinition {
        let is_nullable = match &field.ty {
            Type::Path(type_path) => match type_path.path.segments.first() {
                Some(segment) => segment.ident == "Option",
                None => false,
            },
            _ => false,
        };

        FieldDefinition {
            name: field.ident.clone().unwrap().clone(),
            column_name: field.ident.clone().unwrap().clone(),
            field_type: field.ty.clone(),
            is_primary_key: false,
            is_nullable,
            has_default: false,
            keyset_pagination: None,
        }
    }

    pub fn parse(field: &syn::Field) -> FieldDefinition {
        let mut field_definition = Self::new(field);

        match field
            .attrs
            .iter()
            .find(|attr| attr.path().is_ident(ENTITY_ATTRIBUTE))
        {
            Some(Attribute {
                meta: Meta::List(MetaList { tokens, .. }),
                ..
            }) => match syn::parse2::<FieldAttribute>(tokens.clone()) {
                Ok(field_attr) => {
                    field_definition.is_primary_key = field_attr.is_primary_key;
                    field_definition.has_default = field_attr.has_default;
                    field_definition.column_name = Ident::new(
                        &format!(
                            "{}",
                            field_attr
                                .alias
                                .unwrap_or(field_definition.column_name.to_string())
                        ),
                        proc_macro2::Span::call_site(),
                    );

                    field_definition.keyset_pagination = match field_attr.keyset_pagination {
                        Some(pagination_data) => Some(PaginationMetadata {
                            field_name: field_definition.name.clone(),
                            field_type: field_definition.field_type.clone(),
                            column_name: field_definition.column_name.clone(),
                            group: pagination_data.group,
                            // order: pagination_data.order,
                        }),
                        None => None,
                    }
                }
                _ => (),
            },
            _ => (),
        }

        field_definition
    }
}

#[derive(Debug)]
pub struct ObjectDefinition {
    pub table_name: String,
    pub struct_name: String,
    // pub schema_name: Option<String>, //Todo: think a way to provide a schema
    pub fields: Vec<FieldDefinition>,
    pub primary_key_fields: Vec<FieldDefinition>,
    pub pagination_metadata_groups: HashMap<String, Vec<PaginationMetadata>>,
}

impl ObjectDefinition {
    fn new(parsed_struct: &ItemStruct, dbo_attribute_metadata: &DboAttributeMetadata) -> Self {
        let table_name = dbo_attribute_metadata
            .table_name
            .clone()
            .unwrap_or(parsed_struct.ident.to_string());

        ObjectDefinition {
            table_name,
            struct_name: parsed_struct.ident.to_string(),
            fields: vec![],
            primary_key_fields: vec![],
            pagination_metadata_groups: HashMap::new(),
        }
    }

    pub fn parse(input: TokenStream) -> syn::Result<Self> {
        let parsed_struct = syn::parse2::<ItemStruct>(input.into())?;

        //Parse the "dbo" attribute
        let dbo_attribute_metadata = DboAttributeMetadata::extract(&parsed_struct);

        let mut object_definitions = ObjectDefinition::new(&parsed_struct, &dbo_attribute_metadata);

        for field in parsed_struct.fields.iter() {
            let field_definition = FieldDefinition::parse(field);

            match field_definition.keyset_pagination.as_ref() {
                Some(keyset_pagination) => {
                    object_definitions
                        .pagination_metadata_groups
                        .entry(keyset_pagination.group.clone())
                        .or_default()
                        .push(keyset_pagination.clone());
                }
                None => (),
            }

            object_definitions.fields.push(field_definition.clone());

            if field_definition.is_primary_key {
                object_definitions.primary_key_fields.push(field_definition);
            }
        }

        Ok(object_definitions)
    }

    pub fn get_create_dto_struct_ident(&self) -> Ident {
        Ident::new(
            &format!("{}Create", self.struct_name),
            proc_macro2::Span::call_site(),
        )
    }

    pub fn get_update_dto_struct_ident(&self) -> Ident {
        Ident::new(
            &format!("{}Update", self.struct_name),
            proc_macro2::Span::call_site(),
        )
    }

    pub fn get_filter_dto_struct_ident(&self) -> Ident {
        Ident::new(
            &format!("{}Filter", self.struct_name),
            proc_macro2::Span::call_site(),
        )
    }

    pub fn get_primary_key_type_exp(&self) -> Vec<proc_macro2::TokenStream> {
        self.primary_key_fields
            .iter()
            .map(
                |FieldDefinition {
                     name, field_type, ..
                 }| quote! { #name: #field_type },
            )
            .collect()
    }

    pub fn get_struct_ident(&self) -> Ident {
        Ident::new(
            &format!("{}", self.struct_name),
            proc_macro2::Span::call_site(),
        )
    }

    pub fn get_page_pagination_struct_ident(&self) -> Ident {
        Ident::new(&format!("PagePagination"), proc_macro2::Span::call_site())
    }

    pub fn get_keyset_pagination_dto_ident(&self, pagination_name: &String) -> Ident {
        let pagination_struct_name = to_pascal_case(pagination_name);

        let keyset_pagination_struct_ident =
            create_ident(&format!("PaginationCursor{}", pagination_struct_name));
        keyset_pagination_struct_ident
    }
}

//Pagination
#[derive(Debug, Clone)]
pub struct PaginationMetadata {
    pub field_name: Ident,
    pub column_name: Ident,
    pub field_type: Type,
    pub group: String,
    // pub order: String,
}

#[derive(Debug)]
pub struct PaginatedByAttrData {
    pub group: String,
    // pub order: String,
}

impl Default for PaginatedByAttrData {
    fn default() -> Self {
        Self {
            group: "keyset_pagination".to_string(),
            // order: "asc".to_string(),
        }
    }
}

impl PaginatedByAttrData {
    fn new(group: String) -> Self {
        PaginatedByAttrData {
            group,
            // order: "asc".to_string(),
        }
    }
}
