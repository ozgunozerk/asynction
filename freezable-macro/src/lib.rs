//! #[freezable] and `freeze!()` macro implementations
//! freezable-macro is not smart yet (and probably will never be). You have to write types explicitly for this macro to work.
//! we need the types, so that we can store the variables in state machines (enum chunks).
//! So, for example, the code should be like this:
//!
//! ```ignore
//! fn freezable_complex(begin: u8) -> String {
//!     let current_num: u8 = begin;
//!     freeze!();  // freezes the function, and returns no partial value
//!
//!     let (num1, num2): (u8, u8) = (current_num + 1, current_num - 1);
//!     freeze!();
//!
//!     let mut mult_str: String = (num1 * num2).to_string();
//!     freeze!();
//!
//!     mult_str.push_str(" a random text");
//!     mult_str.truncate(10);
//!     mult_str
//! }
//! ```
//!
//! the feature of deriving the type implicitly could be implemented to our macro, but it won't be anything original,
//! nor directly related to the main problem of demystifying the concept of asynchronous code.
//! It will just be a duplication of the compiler's work.
//! I want to include only the essential logic for generating an asynchronous context into the freezable-macro.
//! It will be expected that, the freezable-macro will lack many optimizations/features for the sake of minimalism,
//! in the end, it is not an end product, but a learning tool to discover the concepts: async, generators, yield, etc...

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned};

#[proc_macro_attribute]
pub fn freezable(args: TokenStream, input: TokenStream) -> TokenStream {
    assert!(args.is_empty());
    let ty = parse_macro_input!(input as syn::Item);
    freezable_2(ty)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

fn freezable_2(input: syn::Item) -> Result<proc_macro2::TokenStream, syn::Error> {
    if let syn::Item::Fn(item) = input {
        let return_type = parse_return_type(&item);
        println!("return type is:  {:#?}", return_type);

        let mut chunks = vec![vec![]];

        item.block
            .stmts
            .iter()
            .for_each(|statement| match statement {
                syn::Stmt::Semi(e, t) => {
                    if quote!(#e).to_string().starts_with("freeze! (") {
                        chunks.push(vec![]);
                    } else {
                        chunks
                            .last_mut()
                            .unwrap()
                            .push(syn::Stmt::Semi(e.clone(), *t))
                    }
                }
                syn::Stmt::Local(local) => parse_variable_names_and_types(local),
                other => chunks.last_mut().unwrap().push(other.clone()),
            });

        let name = item.sig.ident.clone();
        let variants = variant_generator(&chunks);

        Ok(quote! {
            #[allow(non_camel_case_types)]
            pub enum #name {
                #(#variants),*,
                Finished,
                Cancelled,
            }
            #item
        })
    } else {
        Err(syn::Error::new(input.span(), "expected a function!"))
    }
}

fn parse_return_type(input: &syn::ItemFn) -> Option<String> {
    if let syn::ReturnType::Type(_, a) = &input.sig.output {
        if let syn::Type::Path(b) = &**a {
            Some(b.path.segments[0].ident.to_string())
        } else {
            unreachable!("return type should not be empty");
        }
    } else {
        None
    }
}

fn parse_variable_names_and_types(local: &syn::Local) {
    if let syn::Pat::Type(a) = &local.pat {
        if let syn::Pat::Ident(b) = &*a.pat {
            println!("here is name: {:#?}", b.ident.to_string());
            if let syn::Type::Path(c) = &*a.ty {
                println!("here is type: {:#?}", c.path.segments[0].ident.to_string());
            }
        }
        if let syn::Pat::Tuple(b) = &*a.pat {
            for elem in b.elems.iter() {
                if let syn::Pat::Ident(c) = elem {
                    println!("here is name: {:#?}", c.ident.to_string());
                }
            }
            if let syn::Type::Tuple(c) = &*a.ty {
                for elem in c.elems.iter() {
                    if let syn::Type::Path(d) = elem {
                        println!("here is type: {:#?}", d.path.segments[0].ident.to_string());
                    }
                }
            }
        }
    }
}

fn variant_generator(chunks: &[Vec<syn::Stmt>]) -> Vec<proc_macro2::Ident> {
    chunks
        .iter()
        .enumerate()
        .map(|(i, _)| {
            let variant_name_str = format!("chunk{}", i);
            syn::Ident::new(&variant_name_str, proc_macro2::Span::call_site())
        })
        .collect::<Vec<syn::Ident>>()
}
