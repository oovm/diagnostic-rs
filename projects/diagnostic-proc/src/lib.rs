extern crate proc_macro;

use crate::diag::my_macro;

#[proc_macro]
pub fn real_macro(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match my_macro(tokens.into()) {
        Ok(tokens) => tokens.into(),
        Err(diag) => diag.emit_as_expr_tokens().into()
    }
}

mod diag;