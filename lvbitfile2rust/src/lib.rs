extern crate proc_macro;

use quote::quote;

pub fn codegen(bitfile_path: &str) -> Result<proc_macro2::TokenStream, Box<dyn std::error::Error>> {
    let bitfile_contents = std::fs::read_to_string(bitfile_path)?;
    let bitfile = roxmltree::Document::parse(&bitfile_contents)?;
    Ok(quote!{
        mod types {

        }

        struct Registers {

        }
    })
}
