use proc_macro::TokenStream;
use quote::{__private::TokenStream as TokenStream2, quote};
use syn::{self, Data, DeriveInput, FieldsNamed, FieldsUnnamed};

fn impl_for_named_from_kuzu_row(fields: &FieldsNamed) -> TokenStream2 {
    let names = fields
        .named
        .iter()
        .map(|field| (field.ident.clone().unwrap()));

    quote! {
        Self {
            #(
                #names: row.get_val_by_column(stringify!(#names))?
            ),*
        }
    }
}

fn impl_for_unnamed_from_kuzu_row(fields: &FieldsUnnamed) -> TokenStream2 {
    let names = 0..(fields.unnamed.len());

    quote! {
        Self (
            #(
                row.get_val(#names)?
            ),*
        )
    }
}

pub fn from_kuzu_row_derive(item: TokenStream) -> TokenStream {
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
            syn::Fields::Named(ref fields) => impl_for_named_from_kuzu_row(fields),
            syn::Fields::Unnamed(ref fields) => impl_for_unnamed_from_kuzu_row(fields),
            _ => unreachable!(),
        }
    };

    quote! {
        impl TryFrom<Row> for #struct_name {
            type Error = kuzu_rs::error::Error;

            #[inline]
            fn try_from(row: Row) -> Result<Self, Self::Error> {
                Ok(#body)
            }
        }
    }
    .into()
}
