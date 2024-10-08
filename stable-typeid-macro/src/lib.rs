#![feature(proc_macro_span)]
#![feature(proc_macro_def_site)]
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, Ident};
mod util;
use util::*;

#[proc_macro_attribute]
pub fn sort(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let vis = input.vis;
    let name = input.ident;
    let generics = input.generics;
    let expanded: proc_macro2::TokenStream = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => {
            let mut named_fields: Vec<_> = fields.named.iter().collect();
            named_fields.sort_by_key(|f| hash(&f.ident.clone().unwrap().to_string()));
            quote! {
                #vis struct #name #generics {
                    #(#named_fields),*
                }
            }
        }
        Data::Struct(DataStruct {
            fields: Fields::Unnamed(fields),
            ..
        }) => {
            let mut unnamed_fields: Vec<_> = fields
                .unnamed
                .iter()
                .enumerate()
                .map(|(i, _)| i.to_string())
                .collect();
            unnamed_fields.sort_by_key(|f| hash(&f));
            quote! {
                #vis struct #name #generics {
                    #(#unnamed_fields),*
                }
            }
        }
        Data::Enum(data) => {
            let mut variants: Vec<_> = data.variants.iter().collect();
            variants.sort_by_key(|v| hash(&v.ident.to_string()));
            quote! {
                #vis enum #name #generics {
                    #(#variants),*
                }
            }
        }
        _ => panic!("Expected a struct or enum"),
    };
    TokenStream::from(expanded)
}
#[proc_macro_derive(StableID)]
pub fn stable_id(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let mut type_string = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => {
                let type_str_list: Vec<String> = fields
                    .named
                    .iter()
                    .map(|f| {
                        format!(
                            "{}: {}",
                            f.ident.as_ref().unwrap().to_string(),
                            f.ty.to_token_stream().to_string()
                        )
                    })
                    .collect();
                format!("struct~{}{{{}}}", name.to_string(), type_str_list.join(";"))
            }
            Fields::Unit => {
                format!("struct~{}", name.to_string())
            }
            Fields::Unnamed(fields) => {
                let type_str_list: Vec<String> = fields
                    .unnamed
                    .iter()
                    .enumerate()
                    .map(|(i, f)| {
                        f.ident
                            .clone()
                            .map(|x| x.to_string())
                            .unwrap_or(i.to_string())
                            .to_string()
                    })
                    .collect();
                format!("struct~{}({})", name.to_string(), type_str_list.join(","))
            }
        },
        Data::Enum(data) => {
            let type_str_list: Vec<String> = data
                .variants
                .iter()
                .map(|v| {
                    format!(
                        "{}{}",
                        v.ident.to_string(),
                        v.fields.to_token_stream().to_string(),
                    )
                })
                .collect();
            format!("enum~{}{{{}}}", name.to_string(), type_str_list.join(","))
        }
        _ => panic!("Expected a struct or enum"),
    };
    type_string = format!("{}%{}", get_pkg_name(), type_string);
    let hash = hash(&type_string);
    let doc = format!("type_name = {} \ntype_id = {}", type_string, hash);
    let expanded = quote! {
        #[doc = #doc]
        impl stable_typeid::StableAny for #name {
            fn stable_id(&self) -> &'static stable_typeid::StableId where Self: Sized {
                &stable_typeid::StableId(#hash)
            }
        }

        impl stable_typeid::StableID for #name {
            const _STABLE_ID: &'static stable_typeid::StableId = &stable_typeid::StableId(#hash);
        }
    };
    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn stable_sorted_type(attr: TokenStream, item: TokenStream) -> TokenStream {
    let sort = sort(attr, item);
    let stable = stable_id(sort.clone());
    let sort = proc_macro2::TokenStream::from(sort);
    let stable = proc_macro2::TokenStream::from(stable);
    let expanded = quote! {
        #sort
        #stable
    };
    TokenStream::from(expanded)
}
