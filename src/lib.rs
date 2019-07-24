extern crate proc_macro;

use proc_macro2::{
    Ident,
};

use quote::{
    quote,
};

#[proc_macro]
pub fn group_name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let group = syn::parse_macro_input!(input as Ident);

    let appended_group = format!("{}_GROUP", group.to_string().to_uppercase());
    let ident = Ident::new(&appended_group, group.span());

    let expanded = quote!(#ident);
    proc_macro::TokenStream::from(expanded)
}