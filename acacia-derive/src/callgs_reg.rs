use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, LitStr, parse_macro_input};

pub fn impl_callgs_reg(attr: TokenStream, item: TokenStream) -> TokenStream {
    let api_name = parse_macro_input!(attr as LitStr);
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;

    let expanded = quote! {
        #input
        inventory::submit! {
            crate::base::dispatcher::CallGsHandlerRegistration {
                api_name: #api_name,
                func: |conn: std::sync::Arc<crate::base::conn::Conn>,
                       msgdata: acacia_game_msg::protocol::ReqCallGs|
                       -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<()>> + Send>>
                {
                    Box::pin(async move {
                        let param: serde_json::Value = serde_json::from_str(&msgdata.param)
                            .unwrap_or(serde_json::Value::Null);
                        tracing::debug!("-> callgs api={} param={}", #api_name, param);
                        #fn_name(conn, msgdata, param).await
                    })
                },
                name: stringify!(#fn_name),
            }
        }
    };

    TokenStream::from(expanded)
}
