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

#[proc_macro_attribute]
pub fn rpc(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree
    let input = syn::parse_macro_input!(item as syn::ItemStruct);
    let input_ext = input.clone();
    let ident = &input.ident;
    let generics = &input.generics;
    let generics_ext = &mut input_ext.clone().generics;
    let generics_params_ext = &mut generics_ext.params;
    let static_lifetime = syn::Lifetime::new("'rpc", proc_macro2::Span::call_site());
    generics_params_ext.push(syn::GenericParam::Lifetime(syn::LifetimeParam::new(static_lifetime)));
    // Generate the new function body
    let gen = quote::quote! {
        #input

        type Params = Vec<serde_json::Value>;
        type BoxFuture<'rpc, T> = std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send + 'rpc>>;
        type MethodFunction #generics_ext = fn(Arc<#ident #generics>, Params) -> BoxFuture<'rpc, json_rpc::Response>;

        pub struct Rpc #generics_ext {
          pub methods: HashMap<String, MethodFunction #generics_ext>,
          pub ctx: Arc<#ident #generics>,
          pub ids: HashMap<core::net::SocketAddr, json_rpc::Id>,
        }

        impl #generics_ext Rpc #generics_ext {
          pub fn new(ctx:Arc<#ident #generics>) -> Self {
            Self {
              methods: HashMap::new(),
              ctx,
              ids: HashMap::new(),
            }
          }

          pub fn register_method(&mut self, name: String, method: MethodFunction #generics_ext) {
            self.methods.insert(name, method);
          }

          pub async fn call_method(&self, id: json_rpc::Id, name: String, params: Params) -> json_rpc::Response {
            if let Some(method) = self.methods.get(&name) {
              let mut response = method(self.ctx.clone(), params).await;
              response.set_id(id);
              response
            } else {
              let error = json_rpc::JsonRpcError { code: json_rpc::METHOD_NOT_FOUND, message: "Method not found".to_string(), data: None };
              json_rpc::Response::error(error, Some(id))
            }
          }
        }
    };

    // Convert the generated code into a proc_macro::TokenStream and return it
    gen.into()
}


#[proc_macro_attribute]
pub fn rpc_method(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let mut input = syn::parse_macro_input!(item as syn::ItemFn);
  let mut input_wrapper = input.clone();

  let original_input_ident = &input.sig.ident;
  let input_sig_arg = &input.sig.inputs;

  // Extract argument identifiers
  let arg_names: Vec<_> = input_sig_arg.iter().map(|arg| {
    match arg {
        syn::FnArg::Typed(pat_type) => {
            if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                &pat_ident.ident
            } else {
                panic!("Expected argument to be an identifier");
            }
        }
        syn::FnArg::Receiver(_) => panic!("Expected function argument, not self"),
    }
  }).collect();

  let new_sig_ident = &format!("inner_{}", original_input_ident);
  input.sig.ident = syn::Ident::new(&new_sig_ident, proc_macro2::Span::call_site());
  let input_sig_ident = &input.sig.ident;
  input_wrapper.sig.asyncness = None;

  let input_generics = &input.sig.generics;

  // Check if the function has generics
  let turbofish = if input_generics.params.is_empty() {
    quote::quote! {}
  } else {
      quote::quote! {::#input_generics}
  };


  // Construct the return type
  let return_type: syn::Type = syn::parse_str("std::pin::Pin<Box<dyn std::future::Future<Output = Response> + Send>>").unwrap();

  // Update the function's return type
  input_wrapper.sig.output = syn::ReturnType::Type(
      syn::token::RArrow::default(),
      Box::new(return_type),
  );

  let input_wrapper_sig = input_wrapper.sig;

  quote::quote! {
    #input

    #input_wrapper_sig {
      Box::pin(#input_sig_ident #turbofish(#(#arg_names),*))
    }
  }.into()
}
