use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn entry(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let modified = format!(
        "#[export_name = \"_start_rust\"]
         {}
         ",
        input
    );
    modified.parse().unwrap()
}
