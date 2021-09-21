extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, Fields};

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    println!("the whole struct is {:#?}", ast.ident);
    // println!("data is {:#?}",ast.data);
    let _ = match &ast.data {
        syn::Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(name) => {
                let mut counter = 0;
                for i in name.named.iter() {
                    println!(" ==> the {}'s filed ident is {:#?}",counter , i.ident,);
                    
                    for j in i.attrs.iter(){
                        println!("{} has attribute {:#?}" ,std::iter::repeat("-").take(10).collect::<String>(),
                        j.tokens);
                    }
                    counter+=1;
                }
            }

            Fields::Unnamed(_) => todo!(),
            Fields::Unit => todo!(),
        },
        syn::Data::Enum(data_enum) => {
            println!("{:#?}", data_enum.variants.last().unwrap().ident);
        }
        syn::Data::Union(_) => {}
    };

    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}
