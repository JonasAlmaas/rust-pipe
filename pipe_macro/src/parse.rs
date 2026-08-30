use proc_macro2::{Spacing, Span, TokenStream, TokenTree};
use syn::parse::{Parse, ParseStream};

use crate::Pipe;

impl Parse for Pipe {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let tokens = input.parse::<TokenStream>()?;
        let mut parts = split_pipe(tokens).into_iter();

        let input = parts
            .next()
            .ok_or_else(|| syn::Error::new(Span::call_site(), "expected pipe input"))
            .and_then(syn::parse2::<syn::Expr>)?;

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
