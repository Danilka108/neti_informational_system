use proc_macro::TokenStream;

mod entity;
mod entity_method;

#[proc_macro_attribute]
pub fn entity(args: TokenStream, input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::ItemStruct);
    let gen = entity::impl_entity(ast, args);
    gen.into()
}

#[proc_macro_attribute]
pub fn entity_method(args: TokenStream, input: TokenStream) -> TokenStream {
    entity_method::impl_entity_method(input, args)
}
