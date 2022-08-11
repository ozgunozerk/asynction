extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned};

/// caller function for convenience (responsible for the transition between proc_macro -> proc_macro2)
#[proc_macro_attribute]
pub fn freezable(args: TokenStream, input: TokenStream) -> TokenStream {
    assert!(args.is_empty());
    let ty = parse_macro_input!(input as syn::Item);
    freezable_2(ty)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

/// right now, this macro only detects the `freeze!()` statement, and removes it from the function, as a PoC
fn freezable_2(input: syn::Item) -> Result<proc_macro2::TokenStream, syn::Error> {
    if let syn::Item::Fn(mut item) = input {
        let mut chunks = vec![];

        item.block
            .stmts
            .iter()
            .for_each(|statement| match statement {
                syn::Stmt::Semi(e, t) => {
                    if !quote!(#e).to_string().starts_with("freeze!(") {
                        chunks.push(syn::Stmt::Semi(e.clone(), *t))
                    }
                }
                other => chunks.push(other.clone()),
            });

        item.block.stmts = chunks;
        Ok(quote! (#item))
    } else {
        Err(syn::Error::new(input.span(), "expected a function!"))
    }
}
