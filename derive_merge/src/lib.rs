use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, DeriveInput, Type};

/// 事实上在宏展开的阶段无法获得类型信息
fn type_is_option(ty: &Type) -> bool {
    match ty {
        Type::Path(type_path) => type_path
            .path
            .segments
            .iter()
            .any(|path_segment| path_segment.ident.to_string().eq("Option")),
        _ => false,
    }
}
fn has_attr(attrs: &[Attribute], name: &str) -> bool {
    match attrs.get(0) {
        Some(attr) => attr
            .path
            .segments
            .iter()
            .any(|path| path.ident.to_string().eq(name)),
        None => false,
    }
}

#[proc_macro_derive(MergeProto, attributes(force, exclude))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = TokenStream::from(input);
    let DeriveInput { ident, data, .. } = syn::parse2(input).unwrap();

    let merge_proto = match &data {
        syn::Data::Struct(message) => match message.fields {
            syn::Fields::Named(ref fileds) => {
                let recurse = fileds.named.iter().map(|f| {
                    let name = &f.ident;
                    let ty = &f.ty;
                    let attrs = &f.attrs;

                    if has_attr(attrs, "exclude") {
                        quote!()
                    } else if has_attr(attrs, "force") || !type_is_option(ty) {
                        quote! {
                            self.#name = another.#name.clone();
                        }
                    } else {
                        quote! {
                            if another.#name.is_some() {
                                self.#name = another.#name.clone();
                            }
                        }
                    }
                });
                quote! {
                    #(#recurse)*
                }
            }
            syn::Fields::Unnamed(_) | syn::Fields::Unit => unimplemented!(),
        },
        syn::Data::Enum(_) | syn::Data::Union(_) => unimplemented!(),
    };
    println!("{}", merge_proto);
    let output = quote! {
        impl MergeProto for #ident {
            fn merge_proto (&mut self, another: &Self) {
                #merge_proto
            }
        }
    };

    proc_macro::TokenStream::from(output)
}
