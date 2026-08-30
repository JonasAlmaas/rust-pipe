mod expand;
mod parse;

use quote::quote;

pub(crate) struct Pipe {
    input: syn::Expr,
    stages: Vec<syn::Expr>,
}

#[proc_macro]
pub fn pipe(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let pipe = syn::parse_macro_input!(input as Pipe);
    quote! { #pipe }.into()
}
