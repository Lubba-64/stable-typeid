use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, TypeArray};

/// A procedural macro to implement a stable ID for array types.
#[proc_macro]
pub fn stable_id_impl_array(input: TokenStream) -> TokenStream {
    // Parse the input as an array type
    let input_array = parse_macro_input!(input as TypeArray);

    // Get the element type of the array
    let element_type = &input_array.elem;

    // Generate the type name as a string using std::any::type_name
    let type_name = quote!(std::any::type_name::<#element_type>());

    // Compute the FNV-1a hash of the element type name
    let stable_id_hash = quote! {
        fnv1a_hash_64(#type_name.as_bytes(), None)
    };

    // Generate the implementation for StableID with a generic length `N`
    let expanded = quote! {
        impl<const N: usize> StableID for [#element_type; N] {
            const _STABLE_ID: &'static StableId = &StableId(#stable_id_hash);
        }
    };

    // Convert the expanded quote back to a TokenStream
    TokenStream::from(expanded)
}
