extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, AttributeArgs, Data, DataStruct, DeriveInput, Fields, FieldsNamed, Ident,
};

#[proc_macro_attribute]
pub fn rpc(_args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the input struct
  input
}
