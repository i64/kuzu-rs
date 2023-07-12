mod from_kuzurow;
mod from_kuzustruct;

use proc_macro::TokenStream;

#[proc_macro_derive(FromKuzuStruct)]
pub fn kuzu_struct_derive(item: TokenStream) -> TokenStream {
    from_kuzustruct::from_kuzu_struct_derive(item)
}

#[proc_macro_derive(FromKuzuRow)]
pub fn kuzu_row_derive(item: TokenStream) -> TokenStream {
    from_kuzurow::from_kuzu_row_derive(item)
}
