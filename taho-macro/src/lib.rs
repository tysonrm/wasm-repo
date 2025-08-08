use proc_macro::TokenStream;
use quote::quote;

/// Derives [TryFrom] implementation that applies de/serialisers
#[proc_macro_derive(DeSerializeFrom)]
pub fn serializable_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();
    // Build the trait implementation
    impl_serializable_macro(&ast)
}

fn impl_serializable_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {

        impl TryFrom<&Vec<u8>> for #name {
            type Error = serde_json::Error;
            fn try_from(value: &Vec<u8>) -> Result<#name, Self::Error>{
                match serde_json::from_slice(&value) {
                    Ok(deser) => Ok(deser),
                    Err(e) => Err(e)
                }
            }
        }

        impl From<#name> for Vec<u8> {
            fn from(value: #name) -> Vec<u8> {
                match serde_json::to_string(&value) {
                    Ok(ser) => ser.as_bytes().to_vec(),
                    Err(e) => vec![],
                }
            }
        }
    };

    gen.into()
}
