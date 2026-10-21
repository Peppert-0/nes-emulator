use proc_macro::TokenStream;
use quote::{TokenStreamExt, quote};
use syn::{DataEnum, DataStruct};

#[proc_macro_derive(Builder, attributes(builder))]
pub fn builder_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_builder(&ast)
}

fn impl_builder(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let data = &ast.data;
    let structure = match data {
        syn::Data::Struct(data_struct) => data_struct,
        syn::Data::Enum(data_enum) => {
            panic!("Builder macro cannot be used with enums");
        }
        syn::Data::Union(data_union) => {
            panic!("Builder macro cannot be used with unions");
        }
    };
    let fields = match &structure.fields {
        syn::Fields::Named(fields_named) => fields_named,
        syn::Fields::Unnamed(fields_unnamed) => {
            panic!("Builder macro cannot be used with unnamed fields")
        }
        syn::Fields::Unit => {
            panic!("Builder macro cannot be used with unit structs")
        }
    };
    let builder_fields = fields.named.iter().map(|field| {
        let name = field.ident.as_ref().unwrap();
        let mut value: Option<syn::Expr> = None;
        for attr in &field.attrs {
            if attr.path().is_ident("builder") {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("default") {
                        value = Some(meta.value()?.parse()?);
                    }
                    Ok(())
                })
                .unwrap();
            }
        }
        if let Some(value) = value {
            quote! {
                #name: #value
            }
        } else {
            quote! {}
        }
    });
    let builder_methods = fields.named.iter().map(|field| {
        let name = field.ident.as_ref().unwrap();
        let ty = &field.ty;
        quote! {
            pub fn #name(mut self, value: #ty) -> Self {
                self.#name = value;
                self
            }
        }
    });
    let struct_implementation = quote! {
        impl Builder for #name {
            fn new() -> Self {
                Self {
                    #(#builder_fields,)*
                }
            }
        }
        impl #name {
            #(#builder_methods)*
        }
    };
    struct_implementation.into()
}
