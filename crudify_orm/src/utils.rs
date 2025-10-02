use syn::Ident;

pub fn to_pascal_case(s: &str) -> String {
    s.split(|c: char| !c.is_alphanumeric()) // Split by non-alphanumeric characters
        .filter(|word| !word.is_empty()) // Remove empty splits
        .map(|word| {
            let mut chars = word.chars();
            let first_char = chars.next().unwrap().to_uppercase().to_string(); // Capitalize the first character
            let rest = chars.as_str().to_lowercase(); // Lowercase the rest of the word
            format!("{}{}", first_char, rest)
        })
        .collect::<String>()
}

pub fn create_ident(value: &str) -> Ident {
    Ident::new(value, proc_macro2::Span::call_site())
}
