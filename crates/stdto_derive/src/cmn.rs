pub use core::fmt;
pub use paste::paste;
pub use proc_macro::TokenStream;
pub use quote::quote;
pub use structmeta::StructMeta;
pub use syn::{
    parse_macro_input, parse_quote, Attribute, AttributeArgs, DeriveInput, Error, Expr, ImplItem,
    Lit, LitStr, Meta, MetaList, MetaNameValue, NestedMeta,
};

pub const ROOT: &str = "stdto";

macro_rules! unwrap_error {
    ($expr:expr) => {
        match $expr {
            Ok(value) => value,
            Err(err) => return err.to_compile_error().into(),
        }
    };
}

macro_rules! simple_single_derive {
    ($name:tt, $derive_name:tt) => {
        paste! {
        #[proc_macro_derive($derive_name)]
        pub fn [<to_ $name>](input: TokenStream) -> TokenStream {
            let ast = syn::parse_macro_input!(input as syn::DeriveInput);
            let name = &ast.ident;
            let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
            quote! {
                impl #impl_generics stdto::$derive_name for #name #ty_generics #where_clause {
                }
            }
            .into()
        }

        #[proc_macro_attribute]
        pub fn $name(_: TokenStream, item: TokenStream) -> TokenStream {
            impl_attribute_with_serde(item, Some(parse_quote!(#[derive(stdto::$derive_name)])))
        }
        }
    };
}

// #[inline]
// pub fn has_derives(ast: &DeriveInput, targets: impl AsRef<[&'static str]>) -> bool {
//     let mut targets = targets.as_ref().to_vec();
//     for attr in &ast.attrs {
//         if attr.path.is_ident("derive") {
//             let derives = attr.tokens.to_string();
//             targets.retain(|&x| !derives.contains(x));
//         }
//     }
//     targets.is_empty()
// }
