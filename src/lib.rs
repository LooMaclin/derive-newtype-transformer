#![recursion_limit = "128"]
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(NewtypeDeref)]
pub fn generate_deref_impl(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let gen = impl_deref(&ast);
    gen.parse().unwrap()
}

fn impl_deref(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    match ast.body {
        syn::Body::Struct(ref variant_data) => {
            match variant_data {
                &syn::VariantData::Tuple(ref fields) => {
                    if fields.len() == 1 {
                        let field_type = &fields[0].ty;
                        use quote::Ident;
                        let name_binding = Ident::new(format!("{}Deref", name));
                        quote!{
                            use std::ops::{Deref as #name_binding};
                            impl #name_binding for #name {
                                type Target = #field_type;

                                fn deref(&self) -> &Self::Target {
                                    &self.0
                                }
                            }
                        }
                    } else {
                        panic!("Derive Deref supported only for newtypes!");
                    }
                },
                _ => panic!("Derive Deref for this variant data type not supported!"),
            }
        },
        _ => panic!("Derive Deref for this body type not supported!"),
    }
}