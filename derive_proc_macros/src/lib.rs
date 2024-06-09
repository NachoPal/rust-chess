extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Piece)]
pub fn new_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Get the name of the type we are deriving for
    let name = input.ident;

    // Generate the implementation of the trait
    let expanded = quote! {
        impl Piece for #name {
          fn new(color: Color) -> Self {
            Self(color)
          }
          fn color(&self) -> Color {
            self.0
          }
          // fn valid_moves(&self) -> Vec<MovementKind> {
          //   vec![
          //     Vertical(Forward(8)),
          //   ]
          // }
        }
    };

    // Convert the generated code into a TokenStream and return it
    TokenStream::from(expanded)
}
