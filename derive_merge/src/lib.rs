use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, DeriveInput, Type};
fn type_is_option(ty: &Type) -> bool {
    match ty {
        // Type::Path(path) => path.qself.into_iter().next().unwrap() == "Option",
        _ => true,
    }
}

#[proc_macro_derive(MergeProto, attributes(force))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = TokenStream::from(input);
    let DeriveInput { ident, data, .. } = syn::parse2(input).unwrap();

    let merge = match &data {
        syn::Data::Struct(message) => match message.fields {
            syn::Fields::Named(ref fileds) => {
                let recurse = fileds.named.iter().map(|f| {
                    let name = &f.ident;
                    let ty = &f.ty;
                    let attr = &f.attrs;

                    // TODO
                    if attr.get(0).is_some() || !type_is_option(ty) {
                        quote_spanned! { f.span() =>
                            self.#name = another.#name.clone();
                        }
                    } else {
                        quote_spanned! { f.span() =>
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
    let log = match &data {
        syn::Data::Struct(message) => match message.fields {
            syn::Fields::Named(ref fileds) => {
                let recurse = fileds.named.iter().map(|f| {
                    let name = &f.ident;
                    quote_spanned! { f.span() =>
                        if another.#name.is_some() {
                            self.#name = another.#name.clone();
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

    let output = quote! {
        impl MergeProto for #ident {
            fn merge_proto (&mut self, another: &Self) {
                #merge
            }
        }
    };

    proc_macro::TokenStream::from(output)
}

#[cfg(test)]
mod test {
    use proc_macro::TokenStream;
    use std::str::FromStr;

    #[test]
    fn test() {
        let ts = TokenStream::from_str(
            r"#[derive(MergeProto)]
    struct TestStruct {
        a: Option<i32>,
        b: Option<String>,
        c: Option<u32>,
        #[force]
        d: Option<i32>,
    }",
        )
        .unwrap();

        super::derive(ts);
    }
}
