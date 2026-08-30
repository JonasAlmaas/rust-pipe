use proc_macro2::{Spacing, Span, TokenStream, TokenTree};
use quote::{ToTokens, format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::parse_macro_input;
use syn::visit_mut;
use syn::visit_mut::VisitMut;

#[proc_macro]
pub fn pipe(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let pipe = parse_macro_input!(input as Pipe);
    quote! { #pipe }.into()
}

struct Pipe {
    input: syn::Expr,
    stages: Vec<syn::Expr>,
}

impl Parse for Pipe {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let tokens = input.parse::<TokenStream>()?;
        let mut parts = split_pipe(tokens).into_iter();

        let input = parts
            .next()
            .ok_or_else(|| syn::Error::new(Span::call_site(), "Expected pipe input"))?;

        let input = syn::parse2::<syn::Expr>(input)?;
        let stages = parts
            .map(syn::parse2::<syn::Expr>)
            .collect::<syn::Result<_>>()?;

        Ok(Self { input, stages })
    }
}

fn split_pipe(tokens: TokenStream) -> Vec<TokenStream> {
    let mut result = Vec::new();
    let mut current = TokenStream::new();

    let mut iter = tokens.into_iter().peekable();

    while let Some(token) = iter.next() {
        let is_pipe = matches!(
            (&token, iter.peek()),
            (TokenTree::Punct(a), Some(TokenTree::Punct(b)))
            if a.as_char() == '|'
                && a.spacing() == Spacing::Joint
                && b.as_char() == '>');

        if is_pipe {
            iter.next(); // Consume `>`

            result.push(current);
            current = TokenStream::new();
        } else {
            current.extend(std::iter::once(token));
        }
    }

    result.push(current);
    result
}

impl ToTokens for Pipe {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let pipe_ident = format_ident!("__pipe", span = Span::mixed_site());

        let mut statements = Vec::with_capacity(1 + self.stages.len());

        statements.push({
            let input = &self.input;
            quote! {
                let #pipe_ident = #input;
            }
        });

        for stage in &self.stages {
            let mut stage = stage.clone();
            replace_placeholders(&mut stage, pipe_ident.clone());

            statements.push(quote! {
                let #pipe_ident = #stage;
            });
        }

        tokens.extend(quote! {
            {
                #(#statements)*
                #pipe_ident
            }
        });
    }
}

struct ReplacePlaceholder {
    replacement: syn::Ident,
}

impl VisitMut for ReplacePlaceholder {
    fn visit_expr_mut(&mut self, expr: &mut syn::Expr) {
        if matches!(expr, syn::Expr::Infer(_)) {
            let ident = &self.replacement;
            *expr = syn::parse_quote!(#ident);
            return;
        }

        visit_mut::visit_expr_mut(self, expr);
    }
}

fn replace_placeholders(expr: &mut syn::Expr, replacement: syn::Ident) {
    let mut visitor = ReplacePlaceholder { replacement };
    visitor.visit_expr_mut(expr);
}
