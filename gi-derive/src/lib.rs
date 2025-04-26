use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input, parse_quote};

#[proc_macro_attribute]
pub fn widget(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as DeriveInput);

    if let syn::Data::Struct(ref mut data) = input.data {
        // Add the parent field
        let field: syn::Field = parse_quote! {
            #[doc(hidden)]
            pub(crate) parent: Option<alloc::rc::Rc<std::cell::RefCell<Box<dyn crate::Drawable>>>>
        };

        if let syn::Fields::Named(ref mut fields) = data.fields {
            fields.named.push(field);
        }
    }

    // Add the lifetime if not present
    // if input.generics.lifetimes().next().is_none() {
    //     input.generics.params.insert(0, parse_quote! { 'a });
    // }

    let name = &input.ident;

    TokenStream::from(quote! { 
        #input

        impl Drawable for #name {
            fn as_any(&self) -> &dyn core::any::Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn core::any::Any {
                self
            }

            fn parent(&self) -> Option<&dyn Drawable> {
                todo!()
            }
        
            fn set_parent(&mut self, parent: Box<dyn Drawable>) {
                todo!();
            }
        }
    })
}