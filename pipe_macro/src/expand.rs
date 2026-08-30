use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, format_ident, quote};
use syn::visit_mut;
use syn::visit_mut::VisitMut;

use crate::Pipe;

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
