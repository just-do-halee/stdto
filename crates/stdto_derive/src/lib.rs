#[macro_use]
mod cmn;
use cmn::*;

mod bytes;
use bytes::*;

#[proc_macro_attribute]
pub fn serde(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = syn::parse_macro_input!(item as syn::DeriveInput);
    // check the struct has implemented the Serialize or Deserialize traits from ast.attrs
    let mut has_serialize = false;
    let mut has_deserialize = false;
    for attr in &ast.attrs {
        if attr.path.is_ident("derive") {
            let tokens = attr.tokens.to_string();
            if tokens.contains("Serialize") {
                has_serialize = true;
            }
            if tokens.contains("Deserialize") {
                has_deserialize = true;
            }
        }
    }
    if !has_serialize {
        ast.attrs
            .push(parse_quote! { #[derive(stdto::serde::Serialize)] });
    }
    if !has_deserialize {
        ast.attrs
            .push(parse_quote! { #[derive(stdto::serde::Deserialize)] });
    }
    if !has_serialize || !has_deserialize {
        ast.attrs
            .push(parse_quote! { #[serde(crate = "stdto::serde")] });
    }
    quote!(#ast).into()
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
    let item = impl_attribute_with_serde(item, None);
    let ast = parse_macro_input!(item as DeriveInput);
    let attr = parse_macro_input!(attr as AttributeArgs);
    let options = unwrap_error!(ToBytesOptions::try_from(attr)).to_expr();
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
    quote! {
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
    impl_attribute_with_serde(item, Some(parse_quote!(#[derive(stdto::ToJson)])))
}

fn impl_attribute_with_serde(item: TokenStream, attribute: Option<Attribute>) -> TokenStream {
    let mut ast = syn::parse_macro_input!(item as syn::DeriveInput);
    if ast.attrs.iter().any(|x| x.path.is_ident("serde")) {
        if let Some(attribute) = attribute {
            ast.attrs.push(attribute);
        }
        return quote!(#ast).into();
    }
    let serde: Attribute = parse_quote!(#[stdto::serde]);
    ast.attrs.insert(0, serde);
    if let Some(v) = attribute {
        ast.attrs.push(v);
    }
    quote!(#ast).into()
}
