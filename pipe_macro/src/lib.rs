use pipe_macro_impl::Pipe;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro]
pub fn pipe(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let pipe = parse_macro_input!(input as Pipe);
    quote! { #pipe }.into()
}
