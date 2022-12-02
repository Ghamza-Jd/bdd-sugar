extern crate proc_macro;

use proc_macro::*;

#[proc_macro_attribute]
pub fn given(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn when(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn then(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
