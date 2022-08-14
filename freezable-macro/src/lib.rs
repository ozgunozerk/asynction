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
use proc_macro2::TokenStream as TokenStream2;
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

fn freezable_2(input: Item) -> Result<TokenStream2, syn::Error> {
    if let Item::Fn(func) = input {
        let return_type = parse_return_type(&func);

        let mut code_chunks = vec![vec![]];
        let mut var_chunks = vec![vec![]]; // TODO: try complex types like `Vec<Vec<u8>>` in here, it may not be `Ident`
        let mut freeze_returns: Vec<TokenStream2> = vec![];
        //let return_value;

        if let Some(mut input_vars) = parse_parameters(&func) {
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
                        let value = parse_freeze(e);
                        freeze_returns.push(quote!(#value));
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
        let variants = variant_generator(&var_chunks); // list of variants, along with their types -> `Chunk2(u8, u8)`
        let variant_names = variant_names(&var_chunks); // list of variant names -> `Chunk2`
        let first_chunk_name = variant_names[0].clone(); // necessary for the `start` function
        let parameters = func.sig.inputs; // list of parameters along with their types -> `begin: u8`
        let parameter_names: Vec<Ident> = var_chunks[0] // list of parameter names -> `begin`
            .iter()
            .map(|(name, _type)| name.clone())
            .collect();
        let var_name_chunks = var_chunks
            .iter()
            .map(|inner| {
                inner
                    .iter()
                    .map(|(name, _ty)| name.clone())
                    .collect::<Vec<Ident>>()
            })
            .collect::<Vec<Vec<Ident>>>();

        let match_arms = generate_match_arms(
            &name,
            &variant_names,
            &var_name_chunks,
            &code_chunks,
            &freeze_returns,
            //&return_value,
        );

        Ok(quote! {
            use freezable::{Freezable, FreezableError, FreezableState};

            #[allow(non_camel_case_types)]
            pub enum #name {
                #(#variants,)*
                Finished,
                Cancelled,
            }

            impl #name {
                pub fn start(#parameters) -> Self {
                    #name::#first_chunk_name(#(Some(#parameter_names)),*)
                }
            }

            impl Freezable for #name {
                type Output = #return_type;

                fn unfreeze(&mut self) -> Result<FreezableState<Self::Output>, FreezableError> {
                    match self {
                        #(#match_arms,)*
                        #name::Chunk3(a, _b, _c, _d, _e)=> a.unwrap(),
                        #name::Finished => Err(FreezableError::AlreadyFinished),
                        #name::Cancelled => Err(FreezableError::Cancelled),
                    }
                    // todo!("not yet implemented");
                }


                fn cancel(&mut self) {
                    *self = #name::Cancelled
                }

                fn is_cancelled(&self) -> bool {
                    matches!(self, #name::Cancelled)
                }

                fn is_finished(&self) -> bool {
                    matches!(self, #name::Finished)
                }
            }
        })
    } else {
        Err(syn::Error::new(input.span(), "expected a function!"))
    }
}

fn parse_return_type(func: &syn::ItemFn) -> syn::Type {
    if let syn::ReturnType::Type(_, a) = &func.sig.output {
        *a.clone()
    } else {
        parse_str::<syn::Type>("()").unwrap()
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
                    variant_name_str +=
                        &("Option<".to_owned() + &var_type.to_string() + ">" + ", ");
                    // TODO: actually we need wrap with Option only when the type is not `Copy`, but that will require extra logic
                }
                variant_name_str += ")";
                parse_str::<Variant>(&variant_name_str).unwrap()
            }
        })
        .collect::<Vec<Variant>>()
}

fn variant_names(var_chunks: &[Vec<(Ident, Ident)>]) -> Vec<Variant> {
    var_chunks
        .iter()
        .enumerate()
        .map(|(i, _)| {
            let variant_name_str = format!("Chunk{}", i);
            parse_str::<Variant>(&variant_name_str).unwrap()
        })
        .collect::<Vec<Variant>>()
}

fn generate_match_arms(
    name: &Ident,
    variant_names: &[Variant],
    var_name_chunks: &[Vec<Ident>],
    code_chunks: &[Vec<TokenStream2>],
    freeze_returns: &[TokenStream2],
    //return_value: &[Ident],
) -> Vec<TokenStream2> {
    let mut match_arms = vec![];
    for i in 0..variant_names.len() - 1 {
        let cur_variant_name = &variant_names[i];
        let next_variant_name = &variant_names[i + 1];
        let cur_variable_names = &var_name_chunks[i];
        let next_variable_names = &var_name_chunks[i + 1];
        let cur_code_chunk = &code_chunks[i];
        let cur_freeze_return = &freeze_returns[i];

        // interpolation of Some(5) -> evaluates to 5
        // interpolation of None -> evaluates to nothing
        // hence, the code should be manually written for an Option interpolation
        if freeze_returns[i].is_empty() {
            match_arms.push(quote! {
                #name::#cur_variant_name(#(Some(#cur_variable_names),)*) => {
                    #(let #cur_variable_names = #cur_variable_names.take().expect("value is always present");)*
                    #(#cur_code_chunk);*;
                    *self = #name::#next_variant_name(#(Some(#next_variable_names),)*);
                    Ok(FreezableState::Frozen(None))
                }
            });
        } else {
            match_arms.push(quote! {
                #name::#cur_variant_name(#(Some(#cur_variable_names),)*) => {
                    #(let #cur_variable_names = #cur_variable_names.take().expect("value is always present");)*
                    #(#cur_code_chunk);*;
                    *self = #name::#next_variant_name(#(Some(#next_variable_names),)*);
                    Ok(FreezableState::Frozen(Some(#cur_freeze_return)))
                }
            });
        }
    }

    let last_variant_name = variant_names.last().unwrap();
    let last_variable_names = var_name_chunks.last().unwrap();
    let last_code_chunk = code_chunks.last().unwrap();
    match_arms.push(quote! {
        #name::#last_variant_name(#(Some(#last_variable_names)),*,) => {
            #(let #last_variable_names = #last_variable_names.take().expect("value is always present");)*
            #(#last_code_chunk;)*
            *self = #name::Finished();
            // Ok(FreezableState::Finished(#(#return_value),*))
            Ok(FreezableState::Finished(#(#last_variable_names),*))
        }
    });
    match_arms
}

fn parse_freeze(e: &syn::Expr) -> TokenStream2 {
    if let syn::Expr::Macro(a) = e {
        a.mac.tokens.clone()
    } else {
        unreachable!()
    }
}
