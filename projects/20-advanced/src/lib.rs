use proc_macro::TokenStream;
use quote::quote;

pub trait HelloMacro {
    fn hello_macro();
}

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) ->TokenStream {
    // Construct a representation of Rust code as a syntax tree that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the impl
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generated = quote! {
        impl HelloMacro form #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    };
    generated.into()
}