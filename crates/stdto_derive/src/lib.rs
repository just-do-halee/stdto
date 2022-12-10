use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn serde(_: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    quote! {
        #[derive(stdto::serde::Serialize, stdto::serde::Deserialize)]
        #[serde(crate = "stdto::serde")]
        #ast
    }
    .into()
}

#[proc_macro_derive(ToBytes)]
pub fn to_bytes(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();

    quote! {
        impl #impl_generics stdto::ToBytes for #name #ty_generics #where_clause {
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn bytes(_: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    quote! {
        #[stdto::serde]
        #[derive(stdto::ToBytes)]
        #ast
    }
    .into()
}

#[proc_macro_derive(ToHash)]
pub fn to_hash(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();

    quote! {
        impl #impl_generics stdto::ToHash for #name #ty_generics #where_clause {
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn hash(_: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    quote! {
        #[derive(stdto::ToHash)]
        #ast
    }
    .into()
}
