use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, Attribute, DeriveInput, Type};

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
fn have_force_attr(attrs: &[Attribute]) -> bool {
    match attrs.get(0) {
        Some(attr) => attr.path.segments[0].ident.to_string().eq("force"),
        None => false,
    }
}

#[proc_macro_derive(Merge, attributes(force, ignor))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = TokenStream::from(input);
    let DeriveInput { ident, data, .. } = syn::parse2(input).unwrap();

    let merge = match &data {
        syn::Data::Struct(message) => match message.fields {
            syn::Fields::Named(ref fileds) => {
                let recurse = fileds.named.iter().map(|f| {
                    let name = &f.ident;
                    let ty = &f.ty;
                    let attrs = &f.attrs;

                    if have_force_attr(attrs) || !type_is_option(ty) {
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

    let output = quote! {
        impl Merge for #ident {
            fn merge (&mut self, another: &Self) {
                #merge
            }
        }
    };

    proc_macro::TokenStream::from(output)
}
