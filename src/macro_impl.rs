use quote::quote;
use syn::{parse2, DeriveInput, Result};

pub fn macro_impl(input: proc_macro2::TokenStream) -> Result<proc_macro2::TokenStream> {
    let input: DeriveInput = parse2(input)?;
    let ident_source = &input.ident;

    let expanded = quote! {
        impl #ident_source {}
    };

    Ok(expanded)
}
