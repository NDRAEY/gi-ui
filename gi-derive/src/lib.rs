use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, DeriveInput};

#[proc_macro_attribute]
pub fn with_parent(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as DeriveInput);
    
    if let syn::Data::Struct(ref mut data) = input.data {
        // Add the parent field
        let field: syn::Field = parse_quote! {
            #[doc(hidden)]
            pub(crate) parent: Option<&'a dyn Drawable>
        };
        
        if let syn::Fields::Named(ref mut fields) = data.fields {
            fields.named.push(field);
        }
    }
    
    // Add the lifetime if not present
    if input.generics.lifetimes().next().is_none() {
        input.generics.params.insert(0, parse_quote! { 'a });
    }
    
    TokenStream::from(quote! { #input })
}

#[proc_macro_derive(Widget)]
pub fn as_widget(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    
    // Check if the struct already has lifetime parameters
    let has_lifetimes = match &input.generics {
        syn::Generics { params, .. } => params.iter().any(|param| matches!(param, syn::GenericParam::Lifetime(_))),
    };
    
    let expanded = 
    if !has_lifetimes {
        // If the struct already has lifetimes, use it as-is
        quote! {
            impl Drawable for #name {
                fn as_any(&self) -> &dyn core::any::Any {
                    self
                }
            
                fn as_any_mut(&mut self) -> &mut dyn core::any::Any {
                    self
                }
            }
        }
    } else {
        // If the struct has no lifetimes, add 'static
        quote! {
            impl Drawable for #name<'static> {
                fn as_any(&self) -> &dyn core::any::Any {
                    self
                }
            
                fn as_any_mut(&mut self) -> &mut dyn core::any::Any {
                    self
                }
            }
        }
    };
    
    TokenStream::from(expanded)
}