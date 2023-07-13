extern crate proc_macro;
use quote::{quote, format_ident};
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Data, Field};

#[proc_macro_derive(Builder)]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input as DeriveInput);

    let builder_ident = format_ident!("{}Builder", ident);

    let fields = if let Data::Struct(data) = data { data.fields } else { unimplemented!(); };
    
    let builder_fields = fields.iter().map(|Field { ident, ty, .. }| {
        quote! {
            #ident: Option<#ty>
        }
    });

    let builder_field_initializers = fields.iter().map(|Field { ident, .. }| {
        quote! {
            #ident: None
        }
    });

    let builder_field_setters = fields.iter().map(|Field { ident, ty, .. }| {
        quote! {
            fn #ident(mut self, #ident: #ty) -> Self {
                self.#ident = Some(#ident);
                self
            }
        }
    });

    let builder_field_idents = fields.iter().map(|Field { ident, .. }| {
        quote! {
            #ident
        }
    });
    let builder_field_idents2 = builder_field_idents.clone();
    let builder_field_idents3 = builder_field_idents.clone();
    
    let expanded = quote! {
        impl Built for #ident {
            type BuilderType = #builder_ident;
        
            fn builder() -> Self::BuilderType {
                #builder_ident::new()
            }
        }

        struct #builder_ident {
            #(#builder_fields,)*
        }

        impl #builder_ident {
            #(#builder_field_setters)*
        }
        
        impl Builder for #builder_ident {
            type BuiltType = #ident;
        
            fn new() -> Self {
                #builder_ident { #(#builder_field_initializers,)* }
            }
        
            fn build(self) -> Option<Self::BuiltType> {
                if let (#(Some(#builder_field_idents), )*) = (#(self.#builder_field_idents2, )*) {
                    Some(#ident {
                        #(#builder_field_idents3, )*
                    })
                } else {
                    None
                }
            }
        }
    };

    // println!("{}", &expanded);

    expanded.into()    
}