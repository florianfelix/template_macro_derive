use syn::parse_macro_input;

mod macro_impl;

#[proc_macro_derive(DeriveMacroName)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input);
    macro_impl::macro_impl(input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}
