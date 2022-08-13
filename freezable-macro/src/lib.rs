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
use syn::{parse_macro_input, parse_str, spanned::Spanned, Ident, Item, Variant};

#[proc_macro_attribute]
pub fn freezable(args: TokenStream, input: TokenStream) -> TokenStream {
    assert!(args.is_empty());
    let ty = parse_macro_input!(input as Item);
    freezable_2(ty)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

fn freezable_2(input: Item) -> Result<proc_macro2::TokenStream, syn::Error> {
    if let Item::Fn(func) = input {
        let _return_type = parse_return_type(&func);
        let params = parse_parameters(&func);

        let mut code_chunks = vec![vec![]];
        let mut var_chunks = vec![vec![]];

        if let Some(mut input_vars) = params {
            var_chunks.last_mut().unwrap().append(&mut input_vars);
            var_chunks.push(var_chunks.last().unwrap().clone());
        }

        func.block
            .stmts
            .iter()
            .for_each(|statement| match statement {
                syn::Stmt::Local(local) => {
                    code_chunks.last_mut().unwrap().push(quote!(#statement));

                    var_chunks
                        .last_mut()
                        .unwrap()
                        .append(&mut parse_variable_names_and_types(local));
                }
                syn::Stmt::Semi(e, _t) => {
                    if quote!(#e).to_string().starts_with("freeze! (") {
                        code_chunks.push(vec![]);
                        var_chunks.push(var_chunks.last().unwrap().clone());
                    } else {
                        code_chunks.last_mut().unwrap().push(quote!(#statement));
                    }
                }
                _other => code_chunks.last_mut().unwrap().push(quote!(#statement)),
            });
        var_chunks.pop(); // the last item is not necessary, since we are storing the variables for the next chunk
                          // we don't need to store the variables declared in the last chunk, because there won't be a next chunk

        // for e in code_chunks.iter() {
        //     println!("the statement: {:?}", e);
        // }
        // for e in var_chunks.iter() {
        //     println!("variables: {:?}", e);
        // }

        let name = func.sig.ident.clone();
        let variants = variant_generator(&var_chunks);

        Ok(quote! {
            use freezable::Freezable;

            #[allow(non_camel_case_types)]
            pub enum #name {
                #(#variants),*,
                Finished,
                Cancelled,
            }
            #func
        })
    } else {
        Err(syn::Error::new(input.span(), "expected a function!"))
    }
}

fn parse_return_type(func: &syn::ItemFn) -> Option<Ident> {
    if let syn::ReturnType::Type(_, a) = &func.sig.output {
        if let syn::Type::Path(b) = &**a {
            Some(b.path.segments[0].ident.clone())
        } else {
            unreachable!("return type should not be empty");
        }
    } else {
        None
    }
}

fn parse_parameters(func: &syn::ItemFn) -> Option<Vec<(Ident, Ident)>> {
    if func.sig.inputs.is_empty() {
        return None;
    }

    let mut names_types = vec![];
    for i in func.sig.inputs.iter() {
        if let syn::FnArg::Typed(a) = i {
            if let syn::Pat::Ident(b) = &*a.pat {
                if let syn::Type::Path(c) = &*a.ty {
                    names_types.push((b.ident.clone(), c.path.segments[0].ident.clone()))
                }
            }
        }
    }
    Some(names_types)
}

fn parse_variable_names_and_types(local: &syn::Local) -> Vec<(Ident, Ident)> {
    let mut names_types = vec![];

    // if the statement is a `let` statement
    if let syn::Pat::Type(a) = &local.pat {
        // if it is in format -> `let a = something`
        if let syn::Pat::Ident(b) = &*a.pat {
            if let syn::Type::Path(c) = &*a.ty {
                names_types.push((b.ident.clone(), c.path.segments[0].ident.clone()))
            }
        }

        // if it in tuple format -> `let (a, b, c) = (x, y, z)`
        if let syn::Pat::Tuple(b) = &*a.pat {
            if let syn::Type::Tuple(c) = &*a.ty {
                let names = b.elems.iter();
                let types = c.elems.iter();

                for (name, ty) in names.zip(types) {
                    if let syn::Pat::Ident(c) = name {
                        if let syn::Type::Path(d) = ty {
                            names_types.push((c.ident.clone(), d.path.segments[0].ident.clone()))
                        }
                    }
                }
            }
        }
    }
    names_types
}

fn variant_generator(var_chunks: &[Vec<(Ident, Ident)>]) -> Vec<Variant> {
    var_chunks
        .iter()
        .enumerate()
        .map(|(i, vars)| {
            let mut variant_name_str = format!("Chunk{}", i);
            if vars.is_empty() {
                parse_str::<Variant>(&variant_name_str).unwrap()
            } else {
                variant_name_str += "(";
                for (_, var_type) in vars.iter() {
                    variant_name_str += &(var_type.to_string() + ", ");
                }
                variant_name_str = variant_name_str[..variant_name_str.len() - 2].to_string(); // try to delete this line to see if it still works
                variant_name_str += ")";
                parse_str::<Variant>(&variant_name_str).unwrap()
            }
        })
        .collect::<Vec<Variant>>()
}
