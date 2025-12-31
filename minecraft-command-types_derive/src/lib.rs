use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_derive(HasMacro, attributes(has_macro))]
pub fn has_macro_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let crate_name = std::env::var("CARGO_PKG_NAME").unwrap_or_default();
    let is_internal = crate_name == "minecraft-command-types";
    let crate_path = if is_internal {
        quote! { crate }
    } else {
        quote! { ::minecraft_command_types }
    };

    let has_macro_body = generate_body(&input.data, "has_macro");
    let has_macro_conflict_body = generate_body(&input.data, "has_macro_conflict");

    let expanded = quote! {
        impl #crate_path::has_macro::HasMacro for #name {
            fn has_macro(&self) -> bool {
                #has_macro_body
            }

            fn has_macro_conflict(&self) -> bool {
                #has_macro_conflict_body
            }
        }
    };

    TokenStream::from(expanded)
}

fn should_ignore(attrs: &[Attribute]) -> bool {
    for attr in attrs {
        if attr.path().is_ident("has_macro") {
            let mut ignore = false;
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("ignore") {
                    ignore = true;
                }
                Ok(())
            });
            if ignore {
                return true;
            }
        }
    }
    false
}

fn generate_body(data: &Data, method_name: &str) -> proc_macro2::TokenStream {
    let method = quote::format_ident!("{}", method_name);

    match data {
        Data::Struct(data) => {
            let field_checks = match &data.fields {
                Fields::Named(fields) => {
                    let checks = fields.named.iter().map(|f| {
                        if should_ignore(&f.attrs) {
                            quote! { false }
                        } else {
                            let field_name = f.ident.as_ref().unwrap();
                            quote! { self.#field_name.#method() }
                        }
                    });
                    quote! { #(#checks)||* }
                }
                Fields::Unnamed(fields) => {
                    let checks = fields.unnamed.iter().enumerate().map(|(i, f)| {
                        if should_ignore(&f.attrs) {
                            quote! { false }
                        } else {
                            let index = syn::Index::from(i);
                            quote! { self.#index.#method() }
                        }
                    });
                    quote! { #(#checks)||* }
                }
                Fields::Unit => quote! { false },
            };
            quote! { #field_checks || false }
        }
        Data::Enum(data) => {
            let variant_checks = data.variants.iter().map(|v| {
                let variant_name = &v.ident;
                match &v.fields {
                    Fields::Named(fields) => {
                        let field_names = fields.named.iter().map(|f| f.ident.as_ref().unwrap());

                        let checks = fields.named.iter().map(|f| {
                            let f_name = f.ident.as_ref().unwrap();
                            if should_ignore(&f.attrs) {
                                quote! { false }
                            } else {
                                quote! { #f_name.#method() }
                            }
                        });

                        quote! {
                            Self::#variant_name { #(#field_names),* } => {
                                #(#checks)||* || false
                            }
                        }
                    }
                    Fields::Unnamed(fields) => {
                        let field_bindings: Vec<_> = fields
                            .unnamed
                            .iter()
                            .enumerate()
                            .map(|(i, _)| quote::format_ident!("field_{}", i))
                            .collect();

                        let checks =
                            fields
                                .unnamed
                                .iter()
                                .zip(&field_bindings)
                                .map(|(f, binding)| {
                                    if should_ignore(&f.attrs) {
                                        quote! { false }
                                    } else {
                                        quote! { #binding.#method() }
                                    }
                                });

                        quote! {
                            Self::#variant_name(#(#field_bindings),*) => {
                                #(#checks)||* || false
                            }
                        }
                    }
                    Fields::Unit => quote! { Self::#variant_name => false },
                }
            });
            quote! {
                match self {
                    #(#variant_checks),*
                }
            }
        }
        Data::Union(_) => panic!("HasMacro derive macro cannot be used on unions"),
    }
}
