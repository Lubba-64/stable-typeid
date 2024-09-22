use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

/// A procedural macro to implement a stable ID for a given type.
#[proc_macro]
pub fn stable_id_impl(input: TokenStream) -> TokenStream {
    // Parse the input as a TypePath
    let input_type = parse_macro_input!(input as syn::TypePath);

    // Extract the generics from the last segment of the type path
    let last_segment = input_type
        .path
        .segments
        .last()
        .expect("Expected at least one segment");
    let generics = match &last_segment.arguments {
        syn::PathArguments::AngleBracketed(angle_bracketed) => &angle_bracketed.args,
        _ => panic!("Expected angle-bracketed generic arguments"),
    };

    // Create a new Generics object and populate it with the extracted generics
    let mut parsed_generics = syn::Generics::default();
    for generic in generics {
        // Handle each GenericArgument::Type and convert it to a TypeParam
        if let syn::GenericArgument::Type(ty) = generic {
            parsed_generics
                .params
                .push(syn::GenericParam::Type(syn::TypeParam {
                    attrs: Vec::new(),
                    ident: match ty {
                        syn::Type::Path(type_path) => {
                            type_path.path.segments.last().unwrap().ident.clone()
                        }
                        _ => panic!("Only type paths are supported"),
                    },
                    colon_token: None,
                    bounds: syn::punctuated::Punctuated::new(),
                    eq_token: None,
                    default: None,
                }));
        }
    }

    // Extract just the path (without generics) for the `impl` block
    let type_path_without_generics = &input_type.path;

    // Generate the type name as a string using std::any::type_name
    let type_name = quote!(std::any::type_name::<#type_path_without_generics>());

    // Compute the FNV-1a hash of the type name
    let stable_id_hash = quote! {
        fnv1a_hash_64(#type_name.as_bytes(), None)
    };

    // Split the extracted generics into the parts needed for the impl block
    let (impl_generics, _, where_clause) = parsed_generics.split_for_impl();

    // Generate the implementation
    let expanded = quote! {
        impl #impl_generics StableID for #type_path_without_generics #where_clause {
            const _STABLE_ID: &'static StableId = &StableId(#stable_id_hash);
        }
    };

    // Convert the expanded quote back to a TokenStream
    TokenStream::from(expanded)
}
