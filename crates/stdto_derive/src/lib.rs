use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn bytes(_: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);

    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();

    quote! {
        #[derive(stdto::serde::Serialize, stdto::serde::Deserialize)]
        #[serde(crate = "stdto::serde")]
        #ast
        impl #impl_generics stdto::ToBytes for #name #ty_generics #where_clause {
        }
    }
    .into()
}
