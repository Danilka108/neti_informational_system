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

    let mut entity_attrs = Vec::new();
    let mut entity_attrs_impl = Vec::new();
    let mut entity_non_id_attrs = Vec::new();

    for field in fields {
        let field_name = field
            .ident
            .clone()
            .unwrap()
            .to_string()
            .to_case(Case::Snake);

        let entity_attr = format_ident!("{}", field_name.to_string().to_case(Case::UpperCamel));

        entity_attrs.push(quote! { #entity_attr, });
        entity_attrs_impl.push(quote! {
            Self::#entity_attr => #field_name,
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
            id = Some((entity_attr, field.ident.clone(), field.ty.clone()));
            field.ty = parse_quote! { #id_alias_name };
        } else {
            entity_non_id_attrs.push(entity_attr);
        }

        field.attrs = non_id_attrs;
    }

    let Some((id_entity_attr, id_ident, id_type)) = id else {
        panic!("No field marked with #[id] found");
    };

    let gen = quote! {
        #vis type #id_alias_name = ::utils::entity::Id<#name>;

        #ast

        #[derive(::std::cmp::PartialEq, ::std::cmp::Eq, ::std::hash::Hash)]
        #vis enum #attr_enum_name {
            #(#entity_attrs)*
        }

        impl ::utils::entity::AttrTrait for #attr_enum_name {
            fn name(&self) -> &'static str {
                match self {
                    #(#entity_attrs_impl)*
                }
            }
        }

        impl ::utils::entity::EntityTrait for #name {
            const NAME: &'static str = #name_snake_case;

            type Attr = #attr_enum_name;
            type IdValue = #id_type;

            fn id_attr() -> Self::Attr {
                #attr_enum_name::#id_entity_attr
            }

            fn non_id_attrs() -> Vec<Self::Attr> {
                ::std::vec![#(#attr_enum_name::#entity_non_id_attrs),*]
            }
        }

        impl ::utils::entity::ProvideId<#name> for #name {
            fn provide_id(&self) -> &::utils::entity::Id<#name> {
                &self.#id_ident
            }
        }
    };

    gen
}
