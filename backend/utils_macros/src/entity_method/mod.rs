/// used code: https://github.com/dtolnay/async-trait
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

mod args;
mod expand;
mod lifetime;
mod receiver;

pub fn impl_entity_method(
    input: proc_macro::TokenStream,
    args: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut input = parse_macro_input!(input as syn::ImplItemFn);
    let args = parse_macro_input!(args as args::Args);

    expand::expand(args.ctx_arg_name, &mut input);

    TokenStream::from(quote!(#input))
}
