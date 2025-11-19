mod internal;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn quint_run(args: TokenStream, item: TokenStream) -> TokenStream {
    internal::run::expand(args, item)
}
