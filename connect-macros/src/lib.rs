mod internal;

use proc_macro::TokenStream;

#[proc_macro]
pub fn switch(input: TokenStream) -> TokenStream {
    internal::switch::expand(input)
}

#[proc_macro_attribute]
pub fn quint_run(args: TokenStream, item: TokenStream) -> TokenStream {
    internal::run::expand(args, item)
}

#[proc_macro_attribute]
pub fn quint_test(args: TokenStream, item: TokenStream) -> TokenStream {
    internal::test::expand(args, item)
}
