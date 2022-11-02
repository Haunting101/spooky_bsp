use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::quote;
use syn::{parse_macro_input, parse_quote, Data, DeriveInput, Fields, Type};

#[proc_macro_derive(Decode, attributes(encoding, null_terminated))]
pub fn decode(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match input.data {
        Data::Struct(data_struct) => {
            let name = input.ident;

            let (field_names, field_types) = match data_struct.fields {
                Fields::Named(fields) => fields
                    .named
                    .into_iter()
                    .map(|field| {
                        let field_name = field.ident.unwrap();
                        let mut field_type = field.ty.clone();

                        for attribute in field.attrs {
                            if attribute.path.is_ident("encoding") {
                                if let Type::Path(type_path) = &field.ty {
                                    if type_path.path.is_ident("String")
                                        || type_path.path.is_ident("Rgba")
                                    {
                                        let mut tokens = attribute.tokens.into_iter();

                                        match tokens.next() {
                                            Some(token) => match token {
                                                TokenTree::Group(group) if format!("{}", group.stream()) == String::from("i32") => (),
                                                TokenTree::Group(_) => panic!("Only `i32` is allowed as type"),
                                                _ => panic!("Expected `(TYPE)`, found `{}`", token),
                                            },
                                            None => panic!("#[encoding] requires type"),
                                        }

                                        field_type = parse_quote!(I32Encoded<#field_type>);
                                    } else {
                                        panic!(
                                            "#[encoding] can only be used on `String` or `Rgba`"
                                        );
                                    }
                                }
                            }

                            if attribute.path.is_ident("null_terminated") {
                                if let Type::Path(type_path) = &field.ty {
                                    if type_path.path.is_ident("String") {
                                        field_type = parse_quote!(NullTerminated<#field_type>);
                                    } else {
                                        panic!("#[null_terminated] can only be used on `String`");
                                    }
                                }
                            }
                        }

                        (field_name, field_type)
                    })
                    .fold(
                        (Vec::new(), Vec::new()),
                        |(mut field_names_accumulator, mut field_types_accumulator),
                         (field_name, field_type)| {
                            field_names_accumulator.push(field_name);
                            field_types_accumulator.push(field_type);

                            (field_names_accumulator, field_types_accumulator)
                        },
                    ),
                _ => panic!("#[derive(Decode)] is supported only for structs with named fields"),
            };

            TokenStream::from(quote!(
                impl Decode for #name {
                    fn decode(reader: &mut impl std::io::Read, _state: ()) -> Result<Self, crate::DecodeError> {
                        #(
                            let #field_names = <#field_types>::decode(reader, ())?;
                        )*

                        Ok(Self {
                            #(#field_names),*
                        })
                    }
                }
            ))
        }
        _ => panic!("#[derive(Decode)] is supported only for structs"),
    }
}
