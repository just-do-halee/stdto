#[macro_use]
mod cmn;
use cmn::*;

mod bytes;
use bytes::*;

#[proc_macro_attribute]
pub fn serde(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = syn::parse_macro_input!(item as syn::DeriveInput);
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
    let ast = parse_macro_input!(item as syn::DeriveInput);
    quote! {
         #[derive(stdto::ToHash)]
         #ast
    }
    .into()
}

simple_single_derive!(json, ToJson);
simple_single_derive!(yaml, ToYaml);
simple_single_derive!(toml, ToToml);

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

#[proc_macro_derive(DebugBytes)]
pub fn debug_bytes(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
    quote! {
        impl #impl_generics core::fmt::Debug for #name #ty_generics #where_clause {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{:?}", self.to_bytes())
            }
        }
    }
    .into()
}

#[proc_macro_derive(DebugHex)]
pub fn debug_hex(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
    quote! {
        impl #impl_generics core::fmt::Debug for #name #ty_generics #where_clause {
            fn fmt(&self, mut f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                use stdto::ToHex;
                self.to_bytes().try_to_upper_hex_into_with_0x(&mut f).map_err(|_| core::fmt::Error)
            }
        }
    }
    .into()
}

#[derive(StructMeta)]
struct Debug {
    hasher: Expr,
    // mode: Option<Expr>,
}
#[proc_macro_derive(DebugHash, attributes(debug))]
pub fn debug_hash(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let mut args = None;
    for attr in &ast.attrs {
        if attr.path.is_ident("debug") {
            args = Some(unwrap_error!(attr.parse_args::<Debug>()));
        }
    }
    let Some(args) = args else {
        return Error::new_spanned(ast, "missing #[debug(hasher = ...)]")
            .to_compile_error()
            .into();
        };

    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
    let hasher = args.hasher;
    quote! {
        impl #impl_generics core::fmt::Debug for #name #ty_generics #where_clause {
            fn fmt(&self, mut f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "0x{:02X}", self.to_hash::<#hasher>())
            }
        }
    }
    .into()
}
