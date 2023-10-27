#![feature(iterator_try_collect)]

use proc_macro::TokenStream;

use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::{
    custom_keyword,
    parse::Parse,
    parse_macro_input,
    punctuated::Punctuated,
    spanned::Spanned,
    token::{Brace, Comma, Paren},
    DeriveInput, Error, Expr, Field, Fields, Generics, Path, TypePath,
};

const INJECT_FROM_ATTR_NAME: &str = "inject_from";
const MODULE_ATTR_NAME: &str = "module";

#[proc_macro_derive(FromModule, attributes(inject_from, module))]
pub fn derive_from_module(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as FromModuleInput);
    TokenStream::from(input.into_token_stream())
}

struct FromModuleInput {
    name: Ident,
    module_path: ModulePath,
    deps: Dependencies,
    generics: Generics,
}

impl Parse for FromModuleInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let input: DeriveInput = input.parse()?;
        let module_path = ModulePath::try_from_derive_input(&input)?;
        let default_inject_from = parse_deafult_inject_from(&input)?;

        let deps = match input.data {
            syn::Data::Struct(val) => {
                Dependencies::try_from_fields(default_inject_from, val.fields)?
            }
            _ => return Err(Error::new(input.ident.span(), "only structs are allowed")),
        };

        Ok(Self {
            name: input.ident,
            module_path,
            deps,
            generics: input.generics,
        })
    }
}

fn parse_deafult_inject_from(input: &DeriveInput) -> Result<Option<Expr>, Error> {
    for attr in &input.attrs {
        if !attr.path().is_ident(INJECT_FROM_ATTR_NAME) {
            continue;
        }

        return Ok(Some(attr.parse_args::<Expr>()?));
    }

    Ok(None)
}

impl ToTokens for FromModuleInput {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self {
            module_path:
                ModulePath {
                    generics: module_generics,
                    path: module_path,
                },
            name,
            deps,
            generics,
        } = &self;

        let merged_generics = merge_generics(generics.clone(), module_generics.clone());

        let (_, ty_generics, _) = generics.split_for_impl();
        let (impl_generics, _, where_clause) = merged_generics.split_for_impl();

        quote! {
            impl #impl_generics ::di::FromModule<#module_path> for #name #ty_generics #where_clause {
                fn from_module(module: &#module_path) -> #name {
                    use ::di::Module;
                    #name #deps
                }
            }
        }.to_tokens(tokens);
    }
}

fn merge_generics(mut dest: Generics, src: Generics) -> Generics {
    let dest_params = &mut dest.params;
    let dest_where_clause = &mut dest.where_clause;

    let src_params = src.params;
    let src_where_clause = src.where_clause;

    dest_params.extend(src_params.into_iter());

    match (dest_where_clause, src_where_clause) {
        (Some(dest_where_clause), Some(src_where_clause)) => dest_where_clause
            .predicates
            .extend(src_where_clause.predicates.into_iter()),
        (dest_where_clause @ None, Some(src_where_clause)) => {
            let _ = dest_where_clause.insert(src_where_clause);
        }
        _ => (),
    }

    dest
}

mod kw {
    use syn::custom_keyword;
    custom_keyword!(with);
}

struct ModulePath {
    generics: Generics,
    path: Path,
}

impl Parse for ModulePath {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let path: Path = input.parse()?;
        let _with: kw::with = input.parse()?;
        let generics: Generics = input.parse()?;

        Ok(Self { generics, path })
    }
}

impl ModulePath {
    fn try_from_derive_input(input: &DeriveInput) -> Result<Self, Error> {
        let span = input.ident.span();

        for attr in &input.attrs {
            if !attr.path().is_ident(MODULE_ATTR_NAME) {
                continue;
            }

            return attr.parse_args::<Self>();
        }

        Err(Error::new(
            span,
            format!("\"{}\" attribute is required", MODULE_ATTR_NAME,),
        ))
    }
}

struct Dependencies(StructKind, Vec<Dependency>);

enum StructKind {
    Named,
    Unnamed,
    Unit,
}

impl Dependencies {
    fn try_from_fields(default_inject_from: Option<Expr>, fields: Fields) -> Result<Self, Error> {
        let (kind, deps) = match fields {
            Fields::Unit => (StructKind::Unit, Punctuated::new()),
            Fields::Named(named_fileds) => (StructKind::Named, named_fileds.named),
            Fields::Unnamed(unnamed_fields) => (StructKind::Unnamed, unnamed_fields.unnamed),
        };

        Ok(Self(
            kind,
            deps.into_iter()
                .map(|f| Dependency::try_from_field(default_inject_from.clone(), f))
                .try_collect()?,
        ))
    }
}

impl ToTokens for Dependencies {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let mut deps = Punctuated::<&Dependency, Comma>::new();
        deps.extend(self.1.iter());

        match self.0 {
            StructKind::Unit => return,
            StructKind::Named => quote! { { #deps } }.to_tokens(tokens),
            StructKind::Unnamed => quote! { ( #deps ) }.to_tokens(tokens),
        }
    }
}

struct Dependency {
    from: Expr,
    name: Option<Ident>,
}

impl Dependency {
    fn try_from_field(default_inject_from: Option<Expr>, field: Field) -> Result<Self, Error> {
        let field_span = field.span();

        for attr in field.attrs {
            if !attr.path().is_ident(INJECT_FROM_ATTR_NAME) {
                continue;
            }

            return Ok(Self {
                from: attr.parse_args::<Expr>()?,
                name: field.ident,
            });
        }

        if let Some(default_inject_from) = default_inject_from {
            return Ok(Self {
                from: default_inject_from,
                name: field.ident,
            });
        }

        Err(Error::new(
            field_span,
            format!("\"{}\" attribute is required", INJECT_FROM_ATTR_NAME),
        ))
    }
}

impl ToTokens for Dependency {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self { from, name } = self;

        match name {
            Some(name) => quote! { #name: #from.resolve() },
            None => quote! { #from.resolve() },
        }
        .to_tokens(tokens);
    }
}
