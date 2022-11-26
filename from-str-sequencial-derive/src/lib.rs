use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{
    parse_macro_input, Data, DataEnum, DataUnion, DeriveInput, Fields, FieldsNamed, FieldsUnnamed,
    Ident, Type, Variant,
};

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
    let sequenced_fom_str = format!(
        "{}{}",
        first_field.from_str_expr(),
        other_fields
            .into_iter()
            .map(|f| format!(".ok_or_else(|_| {})", f.from_str_expr()))
            .collect::<String>()
    );
    let gen = quote! {
        impl FromStrSequential for #ident {
            type Err = Box<dyn ::std::fmt::Display>;
            fn from_str_sequential(s: &str) -> Result<Self, Self::Err> {
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
    fn from_str_expr(&self) -> TokenStream {
        match self {
            Self::Unit(ident) => {
                // TODO make the error better
                quote! {
                    (__str.to_ascii_lowercase() == stringify!(#ident).to_ascii_lowercase())
                    .then(Self::<#ident>).ok_or("string not matching")
                }
                .into()
            }
            Self::Unnamed { ident, ty } => quote! {
                <#ty>::FromStr(__str).map(Self::<#ident>)
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

// #[proc_macro_derive(Describe)]
// pub fn describe(input: TokenStream) -> TokenStream {
//     let DeriveInput { ident, data, .. } = parse_macro_input!(input);

//     let description = match data {
//     syn::Data::Struct(s) => match s.fields {
//         syn::Fields::Named(FieldsNamed { named, .. }) => {
//         let idents = named.iter().map(|f| &f.ident);
//         format!(
//             "a struct with these named fields: {}",
//             quote! {#(#idents), *}
//         )
//         }
//         syn::Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
//         let num_fields = unnamed.iter().count();
//         format!("a struct with {} unnamed fields", num_fields)
//         }
//         syn::Fields::Unit => format!("a unit struct"),
//     },
//     syn::Data::Enum(DataEnum { variants, .. }) => {
//         let vs = variants.iter().map(|v| &v.ident);
//         format!("an enum with these variants: {}", quote! {#(#vs),*})
//     }
//     syn::Data::Union(DataUnion {
//         fields: FieldsNamed { named, .. },
//         ..
//     }) => {
//         let idents = named.iter().map(|f| &f.ident);
//         format!("a union with these named fields: {}", quote! {#(#idents),*})
//     }
//     };

//     let output = quote! {
//     impl #ident {
//         fn describe() {
//         println!("{} is {}.", stringify!(#ident), #description);
//         }
//     }
//     };

//     output.into()
// }
