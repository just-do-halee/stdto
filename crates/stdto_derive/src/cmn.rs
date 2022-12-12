pub use core::fmt;
pub use proc_macro::TokenStream;
pub use quote::quote;
pub use syn::{
    parse_macro_input, AttributeArgs, DeriveInput, Error, Expr, Lit, Meta, MetaNameValue,
    NestedMeta,
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
