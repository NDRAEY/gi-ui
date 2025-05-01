use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, TypeParamBound, parse_macro_input, parse_quote};

#[proc_macro_attribute]
pub fn widget(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as DeriveInput);

    if let syn::Data::Struct(ref mut data) = input.data {
        // Add the parent field
        let field: syn::Field = parse_quote! {
            #[doc(hidden)]
            pub(crate) parent: Option<alloc::rc::Weak<core::cell::RefCell<dyn crate::Drawable>>>
        };

        if let syn::Fields::Named(ref mut fields) = data.fields {
            fields.named.push(field);
        }
    }

    let name = &input.ident;
    let mut generics_with_bounds = input.generics.clone();

    // Add `'static` to trait generics.
    for param in &mut generics_with_bounds.params {
        if let syn::GenericParam::Type(type_param) = param.clone() {
            let ident = &type_param.ident;
            let bounds = &type_param.bounds;

            if bounds.iter().any(|a| {
                if let TypeParamBound::Trait(_bnd) = a {
                    true
                } else {
                    false
                }
            }) {
                *param = parse_quote!(#ident: #bounds + 'static);
            }
        }
    }

    let mut generics = generics_with_bounds.clone();

    for param in &mut generics.params {
        if let syn::GenericParam::Type(type_param) = param.clone() {
            let ident = &type_param.ident;

            *param = parse_quote!(#ident);
        }
    }

    let needed_impl = quote! {
        impl #generics_with_bounds Drawable for #name #generics {
            fn as_any(&self) -> &dyn core::any::Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn core::any::Any {
                self
            }

            fn parent(&self) -> Option<alloc::rc::Weak<core::cell::RefCell<dyn Drawable>>> {
                self.parent.as_ref().map(|a| a.clone())
            }

            fn set_parent(&mut self, parent: alloc::rc::Weak<core::cell::RefCell<dyn Drawable>>) {
                self.parent = Some(parent);
            }
        }
    };

    TokenStream::from(quote! {
        #input

        #needed_impl
    })
}
