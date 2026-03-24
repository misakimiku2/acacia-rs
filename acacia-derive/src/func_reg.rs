use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, ItemFn, parse_macro_input};

pub fn impl_func_reg(attr: TokenStream, item: TokenStream) -> TokenStream {
    let msg_id_expr = parse_macro_input!(attr as Expr);
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let msg_type = extract_msg_type(&input.sig).expect("func_reg: expected protocol::*");

    let expanded = quote! {
        #input
        inventory::submit! {
            crate::base::dispatcher::HandlerRegistration {
                msg_id: #msg_id_expr as i32,
                func: |conn: std::sync::Arc<crate::base::conn::Conn>, buf: Vec<u8>| -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<()>> + Send>> {
                    Box::pin(async move {
                        use prost::Message as _;
                        let msgdata = <#msg_type>::decode(buf.as_slice())?;
                        tracing::debug!(
                            "-> {:?} {:?}",
                            Pf::try_from(#msg_id_expr as i32).unwrap_or(acacia_game_msg::protocol::Pf::None),
                            msgdata
                        );
                        #fn_name(conn, msgdata).await
                    })
                },
                name: stringify!(#fn_name),
            }
        }
    };

    TokenStream::from(expanded)
}

fn extract_msg_type(sig: &syn::Signature) -> Option<proc_macro2::TokenStream> {
    let arg = sig.inputs.iter().nth(1)?;
    match arg {
        syn::FnArg::Typed(pat_type) => {
            let ty = &*pat_type.ty;
            match ty {
                syn::Type::Reference(type_ref) => {
                    let inner = &*type_ref.elem;
                    Some(quote! { #inner })
                }
                _ => Some(quote! { #ty }),
            }
        }
        _ => None,
    }
}
