use proc_macro::TokenStream;
mod cif_macro;

#[proc_macro]
pub fn c_if(input: TokenStream) -> TokenStream{
    cif_macro::process(input)
}