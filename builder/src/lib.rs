use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident, DataStruct, Fields, FieldsNamed, Data};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let ident = ast.ident;
    let struct_name = format!("{}Builder", &ident);
    let struct_ident = Ident::new(&struct_name, ident.span());

    let struct_fields = 
        if let Data::Struct( DataStruct { fields: Fields::Named(FieldsNamed { ref named, .. }), .. }) = ast.data {
            named
        } else {
            unimplemented!()
        };

    let optionized_struct_fields = struct_fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;

        quote! { #name: Option<#ty>  }
    });
    
    let methods = struct_fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;

        quote! {
            fn #name (&mut self, #name: #ty) -> &mut Self {
                self.#name = Some(#name);
                self
            }
        }
    });

    let build_method_fields = struct_fields.iter().map(|f| {
        let name = &f.ident; 
        quote! {
            #name: self.#name.clone().ok_or(concat!(stringify!(#name), " is not set."))?
        }
    });

    let expanded = quote! {
        pub struct #struct_ident {
            #(#optionized_struct_fields,)*
        } 

        impl #struct_ident {
            #(#methods)*

            pub fn build(&mut self) -> Result<#ident, Box<dyn std::error::Error>> {
                Ok(
                    #ident {
                        #(#build_method_fields,)*
                    }
                    )
            }
        }

        impl #ident {
            pub fn builder() -> #struct_ident {
                #struct_ident {
                    executable: None,
                    args: None,
                    env: None,
                    current_dir: None,
                }
            }
        }

        
    };
    
    TokenStream::from(expanded)
}
