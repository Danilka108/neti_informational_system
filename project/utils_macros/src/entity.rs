use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{self, parse_quote};

pub fn impl_entity(mut ast: syn::ItemStruct, _args: TokenStream) -> TokenStream2 {
    let name = ast.ident.clone();
    let name_snake_case = name.to_string().to_case(Case::Snake);
    let vis = ast.vis.clone();

    let attr_enum_name = format_ident!("{}Attr", name);
    let id_alias_name = format_ident!("{}Id", name);

    let fields = match &mut ast.fields {
        syn::Fields::Named(fields) => &mut fields.named,
        _ => panic!("Entity macro only supports named fields"),
    };

    let mut id = None;
    let mut enum_variants = Vec::new();
    let mut enum_variants_impl = Vec::new();

    for field in fields {
        let field_name = field
            .ident
            .clone()
            .unwrap()
            .to_string()
            .to_case(Case::Snake);

        let attr_enum_variant =
            format_ident!("{}", field_name.to_string().to_case(Case::UpperCamel));

        enum_variants.push(quote! { #attr_enum_variant, });
        enum_variants_impl.push(quote! {
            Self::#attr_enum_variant => #field_name,
        });

        let (id_attrs, non_id_attrs): (Vec<_>, Vec<_>) =
            field.attrs.drain(..).partition(|attr| match attr.meta {
                syn::Meta::Path(ref path)
                    if path.segments.len() == 1 && path.segments[0].ident.to_string() == "id" =>
                {
                    true
                }
                _ => false,
            });

        if id_attrs.len() > 1 {
            panic!("field can't be marked with #[id] more than once");
        }

        if id_attrs.len() == 1 && id.is_some() {
            panic!("only one field can be marked with #[id]");
        }

        if id_attrs.len() == 1 && id.is_none() {
            id = Some((field.ident.clone(), field.ty.clone()));
            field.ty = parse_quote! { #id_alias_name };
        }

        field.attrs = non_id_attrs;
    }

    let Some((id_ident, id_type)) = id else {
        panic!("No field marked with #[id] found");
    };

    let gen = quote! {
        #vis type #id_alias_name = ::utils::entity::Id<#name>;

        #ast

        #[derive(PartialEq)]
        #vis enum #attr_enum_name {
            #(#enum_variants)*
        }

        impl ::utils::entity::AttrTrait for #attr_enum_name {
            fn name(&self) -> &'static str {
                match self {
                    #(#enum_variants_impl)*
                }
            }
        }

        impl ::utils::entity::EntityTrait for #name {
            const NAME: &'static str = #name_snake_case;

            type Attr = #attr_enum_name;
            type IdValue = #id_type;
        }

        impl ::utils::entity::ProvideId<#name> for #name {
            fn provide_id(&self) -> &::utils::entity::Id<#name> {
                &self.#id_ident
            }
        }
    };

    gen
}
