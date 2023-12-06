use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse_macro_input, Attribute, Data, DeriveInput, Fields};

#[proc_macro_derive(BaseModel, attributes(id))]
pub fn base_model_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;
    let vis = input.vis;
    let mut fields_with_types = Vec::new();
    let mut fields_names = Vec::new();

    if let Data::Struct(data) = &input.data {
        if let Fields::Named(fields) = &data.fields {
            for field in &fields.named {
                if has_id_attribute(&field.attrs) {
                    if let Some(ident) = &field.ident {
                        let field_type = &field.ty;

                        fields_with_types.push(quote! { #ident: #field_type, });
                        fields_names.push(ident);
                    }
                }
            }
        }
    }

    let iden_name = syn::Ident::new(&format!("{struct_name}Iden"), struct_name.span());
    let model_id_name = syn::Ident::new(&format!("{struct_name}Id"), struct_name.span());

    // Build the output tokens
    let expanded = quote! {
        #[derive(Debug, Clone, modql::field::Fields)]
        #vis struct #model_id_name {
            #(pub #fields_with_types)*
        }

        impl ::std::fmt::Display for #model_id_name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
                let mut fields: Vec<String> = Vec::new();
                #(
                    fields.push(
                        format!("{}: {}", stringify!(#fields_names), self.#fields_names)
                    );
                    fields.push(", ".to_string());
                )*

                let _ = fields.pop();

                write!(f, "{}", fields.into_iter().collect::<String>())
            }
        }

        impl ::lib_base::BaseModel for #struct_name {
            type Id = #model_id_name;

            fn id(&self) -> Self::Id {
                #model_id_name {
                    #(#fields_names: self.#fields_names.clone(),)*
                }
            }

            fn table_ref() -> sea_query::TableRef {
                use sea_query::IntoTableRef;
                #iden_name::Table.into_table_ref()
            }
        }
    };

    // Convert the output tokens back into a TokenStream
    TokenStream::from(expanded)
}

fn has_id_attribute(attrs: &[Attribute]) -> bool {
    for attr in attrs {
        if let Ok(meta) = attr.parse_meta() {
            if let syn::Meta::Path(path) = meta {
                if path.is_ident("id") {
                    return true;
                }
            }
        }
    }
    false
}
