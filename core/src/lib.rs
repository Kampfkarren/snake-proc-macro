extern crate proc_macro;

mod snake;

use proc_macro::{Literal, TokenTree, TokenStream};

#[proc_macro]
pub fn snake(_: TokenStream) -> TokenStream {
    TokenTree::Literal(Literal::usize_unsuffixed(snake::main().expect("snake error"))).into()
}
