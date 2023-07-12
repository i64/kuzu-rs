use proc_macro::TokenStream;
use quote::{__private::TokenStream as TokenStream2, quote};
use syn::{self, Data, DeriveInput, FieldsNamed, Ident, Type};

fn impl_for_named_from_kuzu_struct(fields: &FieldsNamed) -> TokenStream2 {
    let (names, tys): (Vec<Ident>, Vec<Type>) = fields
        .named
        .iter()
        .map(|field| (field.ident.clone().unwrap(), field.ty.clone()))
        .unzip();

    let names_vec = quote! {
        vec![#(stringify!(#names).to_string()),*]
    };
    quote! {
        Self {
            #(
                #names: #tys::decode_kuzuval(
                    inner
                    .get(stringify!(#names))
                    .ok_or(error::Error::ColumnNotFound(
                        stringify!(#names).into(),
                        #names_vec
                    ))?
                    .clone()
                )?
            ),*
        }
    }
}

pub(super) fn from_kuzu_struct_derive(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(item).unwrap();

    let struct_name = ast.ident;

    let struct_iteself = {
        match ast.data {
            Data::Struct(ref _s) => _s,
            _ => unreachable!(),
        }
    };

    let body = {
        match struct_iteself.fields {
            syn::Fields::Named(ref fields) => impl_for_named_from_kuzu_struct(fields),
            _ => unreachable!(),
        }
    };

    quote! {
        impl kuzu_rs::types::decode::Decode for #struct_name {
              fn decode_kuzuval(value: KuzuValue) -> error::Result<Self> {
                    match value {
                        KuzuValue::Struct(inner) => Ok(#body),
                        ty => Err(error::Error::DecodeError(
                            ty.name(),
                            std::any::type_name::<Self>(),
                        )),
                }
              }
        }
    }
    .into()
}
