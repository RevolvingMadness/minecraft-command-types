use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_derive(HasMacro)]
pub fn has_macro_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let has_macro_body = match &input.data {
        Data::Struct(data) => {
            let field_checks = match &data.fields {
                Fields::Named(fields) => {
                    let checks = fields.named.iter().map(|f| {
                        let field_name = f.ident.as_ref().unwrap();
                        quote! { self.#field_name.has_macro() }
                    });
                    quote! { #(#checks)||* }
                }
                Fields::Unnamed(fields) => {
                    let checks = fields.unnamed.iter().enumerate().map(|(i, _)| {
                        let index = syn::Index::from(i);
                        quote! { self.#index.has_macro() }
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
                        let checks = field_names
                            .clone()
                            .map(|f_name| quote! { #f_name.has_macro() });
                        quote! {
                            Self::#variant_name { #(#field_names),* } => {
                                #(#checks)||* || false
                            }
                        }
                    }
                    Fields::Unnamed(fields) => {
                        let field_bindings = fields
                            .unnamed
                            .iter()
                            .enumerate()
                            .map(|(i, _)| quote::format_ident!("field_{}", i));
                        let checks = field_bindings
                            .clone()
                            .map(|f_binding| quote! { #f_binding.has_macro() });
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
    };

    let has_macro_conflict_body = match &input.data {
        Data::Struct(data) => {
            let field_checks = match &data.fields {
                Fields::Named(fields) => {
                    let checks = fields.named.iter().map(|f| {
                        let field_name = f.ident.as_ref().unwrap();
                        quote! { self.#field_name.has_macro_conflict() }
                    });
                    quote! { #(#checks)||* }
                }
                Fields::Unnamed(fields) => {
                    let checks = fields.unnamed.iter().enumerate().map(|(i, _)| {
                        let index = syn::Index::from(i);
                        quote! { self.#index.has_macro_conflict() }
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
                        let checks = field_names
                            .clone()
                            .map(|f_name| quote! { #f_name.has_macro_conflict() });
                        quote! {
                            Self::#variant_name { #(#field_names),* } => {
                                #(#checks)||* || false
                            }
                        }
                    }
                    Fields::Unnamed(fields) => {
                        let field_bindings = fields
                            .unnamed
                            .iter()
                            .enumerate()
                            .map(|(i, _)| quote::format_ident!("field_{}", i));
                        let checks = field_bindings
                            .clone()
                            .map(|f_binding| quote! { #f_binding.has_macro() });
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
    };

    let expanded = quote! {
        impl crate::has_macro::HasMacro for #name {
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
