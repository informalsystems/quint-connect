use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{LitInt, LitStr, meta::ParseNestedMeta, parse::Result};

pub(crate) fn parse_str(meta: &ParseNestedMeta<'_>) -> Result<String> {
    Ok(meta.value()?.parse::<LitStr>()?.value())
}

pub(crate) fn parse_num(meta: &ParseNestedMeta<'_>) -> Result<usize> {
    meta.value()?.parse::<LitInt>()?.base10_parse::<usize>()
}

pub(crate) fn quote_opt_str(opt: &Option<String>) -> TokenStream {
    match opt {
        Some(val) => quote! { Some(#val.to_string()) },
        None => quote! { None },
    }
}

pub(crate) fn quote_opt_lit<T: ToTokens>(opt: &Option<T>) -> TokenStream {
    match opt {
        Some(val) => quote! { Some(#val) },
        None => quote! { None },
    }
}

pub(crate) fn quote_seed(opt: &Option<String>) -> TokenStream {
    match opt {
        Some(seed) => quote! { #seed.to_string() },
        None => quote! {
            quint_connect::runner::random_seed()
        },
    }
}
