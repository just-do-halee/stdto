pub use core::fmt;
pub use proc_macro::TokenStream;
pub use quote::quote;
pub use syn::{
    parse_macro_input, parse_quote, Attribute, AttributeArgs, DeriveInput, Error, Expr, Lit, Meta,
    MetaList, MetaNameValue, NestedMeta,
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
