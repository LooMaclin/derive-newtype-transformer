#![recursion_limit = "128"]
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(NewtypeTransformer)]
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
                        let field_type_origin = &fields[0].ty;
                        use quote::Ident;
                        use quote::ToTokens;
                        use quote::Tokens;
                        let mut field_type_tokens = Tokens::new();
                        field_type_origin.to_tokens(&mut field_type_tokens);
                        let field_type = field_type_tokens.into_string().to_lowercase();
                        let new_method_name_owned = Ident::new(format!("to_{}", field_type));
                        let new_method_name_ref = Ident::new(format!("as_{}", field_type));
                        quote!{
                            impl #name {
                                pub fn #new_method_name_owned(self) -> #field_type_origin {
                                    self.0
                                }

                                pub fn #new_method_name_ref(&self) -> &#field_type_origin {
                                    &self.0
                                }
                            }
                        }
                    } else {
                        panic!("Derive NewtypeTransformer supported only for newtypes!");
                    }
                },
                _ => panic!("Derive NewtypeTransformer for this variant data type not supported!"),
            }
        },
        _ => panic!("Derive NewtypeTransformer for this body type not supported!"),
    }
}