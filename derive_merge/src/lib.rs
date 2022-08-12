use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, DeriveInput, Field, Type};

fn type_is_option(ty: &Type) -> bool {
    // In fact, we cannot get type infomation in macro expand period.
    // We just treat path a token stream
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
    // Attribute has two important fields, for example attribute `#[max = 1]`
    // path: path.segment = [max]
    // tokens: ['=', 1]
    attrs.iter().any(|attr| {
        attr.path
            .segments
            .iter()
            .any(|path| path.ident.to_string().eq(name))
    })
}

// #[proc_macro_derive]: (input: TokenStream) -> TokenStream
// #[proc_macro_attribute]: (attr: TokenStream, item: TokenStream) -> TokenStream
// #[proc_macro]: (input: TokenStream) -> TokenStream
#[proc_macro_derive(MergeProto, attributes(force, exclude))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // 这里将proc_macro::TokenSteam转化成proc_macro2::TokenStream，因为他可以单元测试。
    let input = TokenStream::from(input);
    // ident: Name of the struct or enum
    // data: Data within the struct or enum.
    // Beside those we can get
    // vis: Visibility of the struct or enum.
    // attrs: Attributes tagged on the whole struct or enum.
    // generics: Generics required to complete the definition.
    // It's reasonable and necessary to use `unwrap` in macro code and `expect` is better.
    let DeriveInput { ident, data, .. } = syn::parse2(input).unwrap();
    let merge_proto = match &data {
        syn::Data::Struct(message) => match message.fields {
            // we get all named fields in the struct
            syn::Fields::Named(ref fileds) => {
                let recurse = fileds.named.iter().map(|field| {
                    // ty is the type of the field
                    // colon_token
                    let Field {
                        ident, ty, attrs, ..
                    } = &field;

                    if has_attr(attrs, "exclude") {
                        quote!()
                    } else if has_attr(attrs, "force") || !type_is_option(ty) {
                        // We can write the code we want to generate in `quote!`
                        quote! {
                            self.#ident = another.#ident.clone();
                        }
                    } else {
                        quote! {
                            if another.#ident.is_some() {
                                self.#ident = another.#ident.clone();
                            }
                        }
                    }
                });
                // like declaritive macro, do repetitive operation
                quote! {
                    #(#recurse)*
                }
            }
            // we ignore unnamed field and unit field here
            syn::Fields::Unnamed(_) | syn::Fields::Unit => unimplemented!(),
        },
        // we ignore enum and union here
        syn::Data::Enum(_) | syn::Data::Union(_) => unimplemented!(),
    };

    let output = quote! {
        impl MergeProto for #ident {
            fn merge_proto (&mut self, another: &Self) {
                #merge_proto
            }
        }
    };

    // convert quote to proc_macro::TokenStream
    proc_macro::TokenStream::from(output)
}
