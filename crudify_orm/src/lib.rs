use dto_generators::{
    generate_create_dto, generate_filter_dto, generate_keyset_pagination_structs, generate_page_pagination_dto, generate_update_dto
};
use input::ObjectDefinition;
use methods_generators::{
    create_query_method, delete_query_method, generate_filter_method,
    generate_keyset_pagination_methods, get_query_method, page_pagination_query_method,
    update_query_method,
};
use proc_macro::TokenStream;
use quote::quote;

mod constant;
mod dto_generators;
mod input;
mod methods_generators;
mod utils;

#[proc_macro_derive(Entity, attributes(entity))]
pub fn crudify_entity(input: TokenStream) -> TokenStream {
    let object_definitions = match ObjectDefinition::parse(input) {
        Ok(definitions) => definitions,
        Err(e) => {
            panic!("Error parsing input: {}", e);
        }
    };
    // Generate new DTOs
    let generated_create_dto = generate_create_dto(&object_definitions);

    let generated_update_dto = generate_update_dto(&object_definitions);

    let generated_page_pagination_dto = generate_page_pagination_dto(&object_definitions);

    let filter_dto = generate_filter_dto(&object_definitions);
    
    println!("{:?}", filter_dto.to_string());
    // Generate CRUD operations
    let struct_ident = object_definitions.get_struct_ident();

    let create_method = create_query_method(&object_definitions);

    let get_method = get_query_method(&object_definitions);

    let update_method = update_query_method(&object_definitions);

    let delete_method = delete_query_method(&object_definitions);

    let keyset_pagination_methods = generate_keyset_pagination_methods(&object_definitions);

    let keyset_pagination_structs = generate_keyset_pagination_structs(&object_definitions);

    let page_pagination_methods = page_pagination_query_method(&object_definitions);

    let filter_method = generate_filter_method(&object_definitions);

    let expanded = quote! {

         // Create DTO struct
         #generated_create_dto
         // Update DTO struct
         #generated_update_dto
         //Page pagination dto
         #generated_page_pagination_dto
         // Keyset pagination struct
         #(#keyset_pagination_structs)*

         #filter_dto

         impl #struct_ident {

            #create_method

            #get_method

            #update_method

            #delete_method

            #(#keyset_pagination_methods)*

            #page_pagination_methods

            #filter_method

        }

    };

    // Return the generated token stream
    TokenStream::from(expanded)
}
