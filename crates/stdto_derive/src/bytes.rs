use super::*;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endian {
    Big,
    #[default]
    Little,
    Native,
}
impl fmt::Display for Endian {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{ROOT}::Endian::{self:?}")
    }
}

#[derive(Default)]
pub struct ToBytesOptions {
    pub endian: Endian,
}
impl TryFrom<AttributeArgs> for ToBytesOptions {
    type Error = Error;
    fn try_from(args: AttributeArgs) -> Result<Self, Self::Error> {
        let mut options = ToBytesOptions::default();
        for arg in args {
            match arg {
                NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                    path,
                    lit: Lit::Str(lit),
                    ..
                })) => {
                    if path.is_ident("endian") {
                        options.endian = match lit.value().as_str() {
                            "big" => Endian::Big,
                            "little" => Endian::Little,
                            "native" => Endian::Native,
                            _ => panic!("endian must be `big`, `little` or `native`"),
                        };
                    } else {
                        return Err(Error::new_spanned(
                            path,
                            "unknown option, expected `endian`",
                        ));
                    }
                }
                _ => {
                    return Err(Error::new_spanned(
                        arg,
                        "expected `endian = \"...\"` or borsh",
                    ));
                }
            }
        }
        Ok(options)
    }
}
impl ToBytesOptions {
    pub fn to_expr(&self) -> Expr {
        let Self { endian, .. } = self;
        syn::parse_str(&format!("stdto::ToBytesOptions {{ endian: {endian} }}")).unwrap()
    }
}
