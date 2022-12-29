use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{Data, DataEnum, DeriveInput, Fields, Ident, Type};

#[proc_macro_derive(FromStrSequential)]
pub fn from_str_sequential_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let DeriveInput { ident, data, .. } = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_from_str_sequential(ident, data)
}

fn impl_from_str_sequential(ident: Ident, data: Data) -> TokenStream {
    let Data::Enum(data_enum) = data else {panic!("Only enums are supported")};
    let fields = fields_ident(data_enum);
    let ([first_field], other_fields) = fields.split_at(1) else {panic!("Enum need to have at least one variant")};
    let sequenced_fom_str: proc_macro2::TokenStream = format!(
        "{}{}",
        first_field.to_token_stream(),
        other_fields
            .iter()
            .map(|f| format!(".or_else(|_| {})", f.to_token_stream()))
            .collect::<String>()
    )
    .parse()
    .unwrap();
    let gen = quote! {
        impl FromStrSequential for #ident {
            type Err = String;
            fn from_str_sequential(__str: &str) -> Result<Self, Self::Err> {
                #sequenced_fom_str
            }
        }
    };
    gen.into()
}

// local enum to gather only the info we care about the variants
enum CrateVariant {
    Unit(Ident),
    Unnamed { ident: Ident, ty: Type },
}

impl CrateVariant {
    fn to_token_stream(&self) -> TokenStream {
        match self {
            Self::Unit(ident) => {
                quote! {
                    if __str.to_ascii_lowercase() == stringify!(#ident).to_ascii_lowercase() {
                        Ok(Self::#ident)
                    } else {
                        Err("String not matching variant name (case-insensitive)".to_string())
                    }
                }
                .into()
            }
            Self::Unnamed { ident, ty } => quote! {
                <#ty as ::std::str::FromStr>::from_str(__str).map(Self::#ident).map_err(|e| e.to_string())
            }
            .into(),
        }
    }
}

fn fields_ident(data_enum: DataEnum) -> Vec<CrateVariant> {
    let mut fields_ident = Vec::new();
    for variant in data_enum.variants {
        match variant.fields {
            Fields::Unnamed(ref unnamed_fields) if unnamed_fields.unnamed.len() == 1 => {
                fields_ident.push(CrateVariant::Unnamed {
                    ident: variant.ident,
                    ty: unnamed_fields
                        .unnamed
                        .first()
                        .expect("Unnamed fields should have at least one member")
                        .ty
                        .clone(),
                })
            }
            Fields::Unit => fields_ident.push(CrateVariant::Unit(variant.ident)),
            _ => panic!(""),
        }
    }
    fields_ident
}

#[proc_macro]
pub fn debug_macro(input: TokenStream) -> TokenStream {
    println!("{input:?}");
    input
}
