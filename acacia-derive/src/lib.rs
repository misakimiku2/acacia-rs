mod func_reg;
mod callgs_reg;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn func_reg(attr: TokenStream, item: TokenStream) -> TokenStream {
    func_reg::impl_func_reg(attr, item)
}

#[proc_macro_attribute]
pub fn callgs_reg(attr: TokenStream, item: TokenStream) -> TokenStream {
    callgs_reg::impl_callgs_reg(attr, item)
}
