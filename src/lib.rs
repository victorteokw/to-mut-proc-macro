extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(ToMut)]
pub fn to_mut(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_to_mut(&ast)
}

fn impl_to_mut(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    (quote! {
        impl ToMut for #name {
            fn to_mut(&self) -> &mut #name {
                unsafe {
                    let const_ptr = self as *const #name;
                    let mut_ptr = const_ptr as *mut #name;
                    &mut *mut_ptr
                }
            }
        }
    }).into()
}
