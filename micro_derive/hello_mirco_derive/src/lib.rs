use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(HelloMicros)]
pub fn hello_macro_derive(input : TokenStream) -> TokenStream {
     let ast = syn::parse(input).unwrap();
     impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
     let name = &ast.ident;
     let gen = quote! {
          impl HelloMicros for #name{
               fn hello_macro() {
                    println!("hello,macro! {}",stringify!(#name));
               }
          }
     };
     gen.into()
}
