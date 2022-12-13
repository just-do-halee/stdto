#[macro_use]
mod cmn;
use cmn::*;

mod bytes;
use bytes::*;

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
pub fn bytes(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let attr = parse_macro_input!(attr as AttributeArgs);
    let options = unwrap_error!(ToBytesOptions::try_from(attr)).to_expr();
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
    quote! {
        #[stdto::serde]
        #ast
        impl #impl_generics stdto::ToBytes for #name #ty_generics #where_clause {
            const OPTIONS: stdto::ToBytesOptions = #options;
        }
    }
    .into()
}

#[proc_macro_derive(ToHash)]
pub fn to_hash(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
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
    let ast = syn::parse_macro_input!(item as syn::DeriveInput);
    quote! {
        #[derive(stdto::ToHash)]
        #ast
    }
    .into()
}

#[proc_macro_derive(ToJson)]
pub fn to_json(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
    quote! {
        impl #impl_generics stdto::ToJson for #name #ty_generics #where_clause {
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn json(_: TokenStream, item: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(item as syn::DeriveInput);
    quote! {
        #[derive(stdto::ToJson)]
        #ast
    }
    .into()
}
