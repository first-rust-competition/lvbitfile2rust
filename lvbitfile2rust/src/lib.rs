use quote::quote;
use quote::ToTokens;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CodegenError {
    #[error("Failed to read bitfile {0}: {1}")]
    BitfileRead(String, std::io::Error),
    #[error("Failed to parse bitfile {0}: {1}")]
    BitfileParse(String, roxmltree::Error),
    #[error("<{0}> node has no <{1}> child")]
    MissingChild(String, String),
    #[error("<{0}> node has no children")]
    NoChildren(String),
    #[error("<{0}> node has no text")]
    NoText(String),
    #[error("Unknown bitfile type: {0}")]
    UnknownBitfileType(String),
}

struct HashableTokenStream(proc_macro2::TokenStream);

impl std::cmp::PartialEq for HashableTokenStream {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_string().eq(&other.0.to_string())
    }
}

impl std::cmp::Eq for HashableTokenStream {}

impl std::cmp::PartialOrd for HashableTokenStream {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for HashableTokenStream {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.to_string().cmp(&other.0.to_string())
    }
}

impl std::hash::Hash for HashableTokenStream {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_string().hash(state);
    }
}

impl ToTokens for HashableTokenStream {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens)
    }
}

fn sanitize_ident(ident: &str) -> String {
    ident.replace(".", "_").replace(" ", "_")
}

fn first_child_by_tag<'a, 'b>(
    node: &'a roxmltree::Node<'b, 'b>,
    child_tag: &'a str,
) -> Result<roxmltree::Node<'b, 'b>, CodegenError> {
    Ok(node
        .children()
        .find(|child| child.tag_name().name() == child_tag)
        .ok_or_else(|| {
            CodegenError::MissingChild(node.tag_name().name().to_owned(), child_tag.to_owned())
        })?)
}

fn node_text<'a>(node: &'a roxmltree::Node) -> Result<&'a str, CodegenError> {
    node.text()
        .ok_or_else(|| CodegenError::NoText(node.tag_name().name().to_owned()))
}

fn type_node_to_rust_typedecl(
    type_node: &roxmltree::Node,
) -> Result<proc_macro2::TokenStream, CodegenError> {
    match type_node.tag_name().name() {
        "Boolean" => Ok(quote! {bool}),
        "U8" => Ok(quote! {u8}),
        "U16" => Ok(quote! {u16}),
        "U32" => Ok(quote! {u32}),
        "U64" => Ok(quote! {u64}),
        "I8" => Ok(quote! {i8}),
        "I16" => Ok(quote! {i16}),
        "I32" => Ok(quote! {i32}),
        "I64" => Ok(quote! {i64}),
        "SGL" => Ok(quote! {f32}),
        "DBL" => Ok(quote! {f64}),
        "Array" => {
            let size = syn::LitInt::new(
                node_text(&first_child_by_tag(type_node, "Size")?)?,
                proc_macro2::Span::call_site(),
            );
            let inner_type_node = first_child_by_tag(type_node, "Type")?
                .first_element_child()
                .ok_or_else(|| CodegenError::NoChildren("Type".to_owned()))?;
            let inner_typedecl = type_node_to_rust_typedecl(&inner_type_node)?;
            Ok(quote! {
                [#inner_typedecl; #size]
            })
        }
        "Cluster" | "EnumU8" | "EnumU16" | "EnumU32" | "EnumU64" => {
            let ident = syn::Ident::new(
                &sanitize_ident(node_text(&first_child_by_tag(type_node, "Name")?)?),
                proc_macro2::Span::call_site(),
            );
            Ok(quote! {
                types::#ident
            })
        }
        "FXP" => {
            let signed = syn::LitBool {
                value: node_text(&first_child_by_tag(type_node, "Signed")?)? == "true",
                span: proc_macro2::Span::call_site(),
            };
            let word_length = syn::LitInt::new(
                node_text(&first_child_by_tag(type_node, "WordLength")?)?,
                proc_macro2::Span::call_site(),
            );
            let integer_word_length = syn::LitInt::new(
                node_text(&first_child_by_tag(type_node, "IntegerWordLength")?)?,
                proc_macro2::Span::call_site(),
            );
            Ok(quote! {
                ni_fpga::fxp::FXP<#word_length, #integer_word_length, #signed>
            })
        }
        _ => Err(CodegenError::UnknownBitfileType(
            type_node.tag_name().name().to_owned(),
        )),
    }
}

pub fn codegen(bitfile_path: String) -> Result<proc_macro2::TokenStream, CodegenError> {
    let bitfile_contents = match std::fs::read_to_string(&bitfile_path) {
        Ok(contents) => contents,
        Err(e) => return Err(CodegenError::BitfileRead(bitfile_path, e)),
    };
    let bitfile = match roxmltree::Document::parse(&bitfile_contents) {
        Ok(doc) => doc,
        Err(e) => return Err(CodegenError::BitfileParse(bitfile_path, e)),
    };

    let mut typedefs = std::collections::HashSet::<HashableTokenStream>::new();
    let mut register_defs = Vec::<proc_macro2::TokenStream>::new();
    let mut register_fields = Vec::<proc_macro2::TokenStream>::new();
    let mut register_inits = Vec::<proc_macro2::TokenStream>::new();
    let mut vi_signature = String::new();

    for node in bitfile.root().descendants() {
        match node.tag_name().name() {
            "Cluster" => {
                let ident = syn::Ident::new(
                    &sanitize_ident(node_text(&first_child_by_tag(&node, "Name")?)?),
                    proc_macro2::Span::call_site(),
                );
                let mut fields = Vec::<proc_macro2::TokenStream>::new();
                let type_list_node = first_child_by_tag(&node, "TypeList")?;
                for field_node in type_list_node.children().filter(|child| child.is_element()) {
                    let field_name = syn::Ident::new(
                        node_text(&first_child_by_tag(&field_node, "Name")?)?,
                        proc_macro2::Span::call_site(),
                    );
                    let field_typedecl = type_node_to_rust_typedecl(&field_node)?;
                    fields.push(quote! {
                        pub #field_name: #field_typedecl
                    });
                }
                typedefs.insert(HashableTokenStream(quote! {
                    #[derive(Cluster, Debug)]
                    pub struct #ident {
                        #(#fields),*
                    }
                }));
            }
            _name if _name.starts_with("Enum") => {
                let ident = syn::Ident::new(
                    &sanitize_ident(node_text(&first_child_by_tag(&node, "Name")?)?),
                    proc_macro2::Span::call_site(),
                );
                let mut variants = Vec::<proc_macro2::TokenStream>::new();
                let string_list_node = first_child_by_tag(&node, "StringList")?;
                for variant_node in string_list_node
                    .children()
                    .filter(|child| child.is_element())
                {
                    let variant_ident = syn::Ident::new(
                        &sanitize_ident(node_text(&variant_node)?),
                        proc_macro2::Span::call_site(),
                    );
                    variants.push(quote! {
                        #variant_ident
                    });
                }
                typedefs.insert(HashableTokenStream(quote! {
                    #[derive(Debug, Enum)]
                    pub enum #ident {
                        #(#variants),*
                    }
                }));
            }
            "Register" => {
                if node_text(&first_child_by_tag(&node, "Hidden")?)? == "false" {
                    let ident = syn::Ident::new(
                        &sanitize_ident(node_text(&first_child_by_tag(&node, "Name")?)?),
                        proc_macro2::Span::call_site(),
                    );
                    let offset = syn::LitInt::new(
                        &sanitize_ident(node_text(&first_child_by_tag(&node, "Offset")?)?),
                        proc_macro2::Span::call_site(),
                    );
                    let type_node = first_child_by_tag(&node, "Datatype")?
                        .first_element_child()
                        .ok_or_else(|| CodegenError::NoChildren("Datatype".to_owned()))?;
                    let typedecl = type_node_to_rust_typedecl(&type_node)?;
                    let write_fn = match node_text(&first_child_by_tag(&node, "Indicator")?)? {
                        "false" => quote! {
                            pub fn write(&self, value: &#typedecl) -> Result<(), ni_fpga::Error> {
                                unsafe { SESSION.as_ref() }.unwrap().write(#offset, value)
                            }
                        },
                        _ => quote! {},
                    };
                    register_defs.push(quote! {
                        pub struct #ident {
                            _marker: PhantomData<*const ()>,
                        }

                        impl #ident {
                            pub fn read(&self) -> Result<#typedecl, ni_fpga::Error> {
                                unsafe { SESSION.as_ref() }.unwrap().read(#offset)
                            }
                            #write_fn
                        }
                    });
                    register_fields.push(quote! {
                        pub #ident: #ident
                    });
                    register_inits.push(quote! {
                        #ident: #ident{ _marker: PhantomData }
                    });
                }
            }
            "SignatureRegister" => {
                vi_signature = node_text(&node)?.to_owned();
            }
            _ => {}
        }
    }

    let mut typedefs_sorted = typedefs.iter().collect::<Vec<_>>();
    typedefs_sorted.sort();

    Ok(quote! {
        use std::marker::PhantomData;

        #[no_mangle]
        static mut SESSION: *const ni_fpga::Session = std::ptr::null();

        mod types {
            use ni_fpga_macros::{Cluster, Enum};
            use super::types;
            #(#typedefs_sorted)*
        }

        #(#register_defs)*

        pub struct Peripherals {
            _marker: PhantomData<*const ()>,
            #(#register_fields),*
        }

        impl Peripherals {
            pub fn take(resource: &str) -> Result<Self, Box<dyn std::error::Error>> {
                if unsafe{ !SESSION.is_null() } {
                    Err("Peripherals already in use")?
                } else {
                    let session = ni_fpga::Session::open(
                        #bitfile_path,
                        #vi_signature,
                        resource,
                    )?;
                    unsafe { SESSION = &session as *const ni_fpga::Session; }
                    std::mem::forget(session);
                    Ok(
                        Self{
                            _marker: PhantomData,
                            #(#register_inits),*
                        },
                    )
                }
            }
        }

        impl Drop for Peripherals {
            fn drop(&mut self) {
                std::mem::drop(unsafe { SESSION.as_ref() }.unwrap())
            }
        }
    })
}
