extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, ItemStruct};

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
        }
    };

    // Convert the generated code into a TokenStream and return it
    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn rpc(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(item as ItemStruct);
    let ident = &input.ident;
    let generics = &input.generics;
    // Generate the new function body
    let gen = quote! {
        #input

        type Params = Vec<serde_json::Value>;
        type MethodFunction #generics = fn(Arc<#ident #generics>, Params) -> json_rpc::Response;

        pub struct Rpc #generics {
          pub methods: HashMap<String, MethodFunction #generics>,
          pub ctx: Arc<#ident #generics>,
          pub ids: HashMap<core::net::SocketAddr, json_rpc::Id>,
        }

        impl #generics Rpc #generics {
          pub fn new(ctx:Arc<#ident #generics>) -> Self {
            Self {
              methods: HashMap::new(),
              ctx,
              ids: HashMap::new(),
            }
          }

          pub fn register_method(&mut self, name: String, method: MethodFunction #generics) {
            self.methods.insert(name, method);
          }

          pub fn call_method(&self, id: json_rpc::Id, name: String, params: Params) -> json_rpc::Response {
            if let Some(method) = self.methods.get(&name) {
              let mut response = method(self.ctx.clone(), params);
              response.set_id(id);
              response
            } else {
              let error = json_rpc::JsonRpcError { code: json_rpc::METHOD_NOT_FOUND, message: "Method not found".to_string(), data: None };
              json_rpc::Response::error(error, Some(id))
            }
          }
        }
    };

    // Convert the generated code into a TokenStream and return it
    gen.into()
}
