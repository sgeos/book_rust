extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
  // Construct a representation of Rust code as a syntax tree
  // that we can manipulate
  let ast = syn::parse(input).unwrap();

  // Build the trait implementation
  impl_hello_macro(&ast)
}

// DeriveInput {
//   // --snip--
// 
//   ident: Ident {
//     ident: "PancakesB",
//     span: #0 bytes(95..103)
//   },
//   data: Struct(
//     DataStruct {
//       struct_token: Struct,
//       fields: Unit,
//       semi_token: Some(
//         Semi
//       )
//     }
//   )
// }

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
  let name = &ast.ident;
  let gen = quote! {
    impl HelloMacro for #name {
      fn hello_macro() {
        println!("Hello, Macro! My name is {}!", stringify!(#name));
      }
    }
  };
  gen.into()
}

#[cfg(test)]
mod tests {
  #[test]
  fn success() {
    assert!(true);
  }
}
