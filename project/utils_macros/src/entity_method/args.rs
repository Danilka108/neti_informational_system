use syn::parse::Parse;

pub struct Args {
    pub ctx_arg_name: String,
}

impl Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ctx_arg_name = input.parse::<syn::Ident>()?.to_string();
        Ok(Self { ctx_arg_name })
    }
}
