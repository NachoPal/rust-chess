struct RpcArgs {
    auth_fn: Option<syn::Ident>,
}

impl syn::parse::Parse for RpcArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut auth_fn = None;

        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(syn::Ident) {
                let ident: syn::Ident = input.parse()?;
                input.parse::<syn::Token![=]>()?;
                let lit: syn::Lit = input.parse()?;
                match (ident.to_string().as_str(), lit) {
                    ("auth", syn::Lit::Str(lit_str)) => {
                        auth_fn = Some(syn::Ident::new(&lit_str.value(), lit_str.span()));
                    }
                    _ => return Err(
                        syn::Error::new_spanned(ident, "Unsupported attribute")
                    ),
                }
            } else {
                return Err(lookahead.error());
            }

            if !input.is_empty() {
                input.parse::<syn::Token![,]>()?;
            }
        }
        Ok(RpcArgs { auth_fn })
    }
}

#[proc_macro_attribute]
pub fn rpc(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree
    let input = syn::parse_macro_input!(item as syn::ItemStruct);
    let attrs = syn::parse_macro_input!(attr as RpcArgs);

    let input_ext = input.clone();
    let ident = &input.ident;
    let generics = &input.generics;
    let generics_ext = &mut input_ext.clone().generics;
    let generics_params_ext = &mut generics_ext.params;
    let static_lifetime = syn::Lifetime::new("'rpc", proc_macro2::Span::call_site());
    generics_params_ext.push(syn::GenericParam::Lifetime(syn::LifetimeParam::new(
        static_lifetime,
    )));

    let auth_fn = attrs
        .auth_fn
        .unwrap_or_else(|| syn::Ident::new("default_auth", proc_macro2::Span::call_site()));

    // Generate the new function body
    let gen = quote::quote! {
        #input

        type Params = Vec<serde_json::Value>;
        type BoxFuture<'rpc, T> = std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send + 'rpc>>;
        type MethodFunction #generics_ext 
            = fn(
                core::net::SocketAddr,
                std::sync::Arc<tokio::sync::Mutex<#ident #generics>>,
                Params
            ) -> BoxFuture<'rpc, json_rpc::Response>;

        pub struct Rpc #generics_ext {
            pub methods: HashMap<String, (MethodFunction #generics_ext, bool)>,
            pub ctx: std::sync::Arc<tokio::sync::Mutex<#ident #generics>>,
            pub ids: HashMap<core::net::SocketAddr, json_rpc::Id>,
        }

        impl #generics_ext Rpc #generics_ext {
            pub fn new(ctx: #ident #generics) -> Self {
                Self {
                    methods: HashMap::new(),
                    ctx: std::sync::Arc::new(tokio::sync::Mutex::new(ctx)),
                    ids: HashMap::new(),
                }
            }

            pub fn register_method(&mut self, name: String, method: MethodFunction #generics_ext, auth: bool) {
                self.methods.insert(name, (method, auth));
            }

            pub async fn call_method(
                &self,
                addr: core::net::SocketAddr,
                id: json_rpc::Id,
                name: String,
                params: Params
            ) -> json_rpc::Response {
                if let Some((method, auth)) = self.methods.get(&name) {
                    if *auth && !self.auth(addr).await {
                        let error = json_rpc::JsonRpcError { 
                            code: json_rpc::FAILED_AUTH,
                            message: "Failed authentication".to_string(),
                            data: None
                        };
                        return json_rpc::Response::error(error, Some(id));
                    }
                    let mut response = method(addr, self.ctx.clone(), params).await;
                    response.set_id(id);
                    response
                } else {
                    let error = json_rpc::JsonRpcError {
                        code: json_rpc::METHOD_NOT_FOUND,
                        message: "Method not found".to_string(),
                        data: None
                    };
                    json_rpc::Response::error(error, Some(id))
                }
            }

            pub async fn auth(&self, addr: core::net::SocketAddr) -> bool {
                #auth_fn(self, addr).await
            }
        }

        async fn default_auth(_rpc: &Rpc<'_>, _addr: core::net::SocketAddr) -> bool {
            true
        }
    };

    // Convert the generated code into a proc_macro::TokenStream and return it
    gen.into()
}

#[proc_macro_attribute]
pub fn rpc_method(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut input = syn::parse_macro_input!(item as syn::ItemFn);
    let mut input_wrapper = input.clone();

    let original_input_ident = &input.sig.ident;
    let input_sig_arg = &input.sig.inputs;

    // Extract argument identifiers
    let arg_names: Vec<_> = input_sig_arg
        .iter()
        .map(|arg| match arg {
            syn::FnArg::Typed(pat_type) => {
                if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                    &pat_ident.ident
                } else {
                    panic!("Expected argument to be an identifier");
                }
            }
            syn::FnArg::Receiver(_) => panic!("Expected function argument, not self"),
        })
        .collect();

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
    let return_type: syn::Type =
        syn::parse_str("std::pin::Pin<Box<dyn std::future::Future<Output = Response> + Send>>")
            .unwrap();

    // Update the function's return type
    input_wrapper.sig.output =
        syn::ReturnType::Type(syn::token::RArrow::default(), Box::new(return_type));

    let input_wrapper_sig = input_wrapper.sig;

    quote::quote! {
        #input
        pub #input_wrapper_sig {
            Box::pin(#input_sig_ident #turbofish(#(#arg_names),*))
        }
    }
    .into()
}
