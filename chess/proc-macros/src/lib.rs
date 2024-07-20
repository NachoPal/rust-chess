extern crate proc_macro;

#[proc_macro_derive(Piece)]
pub fn new_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    // Get the name of the type we are deriving for
    let name = input.ident;

    // Generate the implementation of the trait
    let expanded = quote::quote! {
        impl Piece for #name {
          fn new(color: Color) -> Self {
            Self(color)
          }
          fn color(&self) -> Color {
            self.0
          }
        }
    };

    // Convert the generated code into a proc_macro::TokenStream and return it
    proc_macro::TokenStream::from(expanded)
}
