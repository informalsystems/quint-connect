use crate::internal::utils::{parse_num, parse_str, quote_opt_lit, quote_opt_str, quote_seed};
use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, meta::ParseNestedMeta, parse::Result, parse_macro_input};

#[derive(Default, Debug)]
struct TestAttrs {
    pub spec: Option<String>,
    pub main: Option<String>,
    pub test: Option<String>,
    pub max_samples: Option<usize>,
    pub seed: Option<String>,
}

impl TestAttrs {
    fn parse(&mut self, meta: ParseNestedMeta) -> Result<()> {
        if meta.path.is_ident("spec") {
            self.spec = Some(parse_str(&meta)?);
        } else if meta.path.is_ident("main") {
            self.main = Some(parse_str(&meta)?);
        } else if meta.path.is_ident("test") {
            self.test = Some(parse_str(&meta)?);
        } else if meta.path.is_ident("max_samples") {
            self.max_samples = Some(parse_num(&meta)?);
        } else if meta.path.is_ident("seed") {
            self.seed = Some(parse_str(&meta)?);
        } else {
            return Err(meta.error("Invalid attribute"));
        }
        Ok(())
    }
}

pub(crate) fn expand(args: TokenStream, item: TokenStream) -> TokenStream {
    let mut attrs = TestAttrs::default();
    let parser = syn::meta::parser(|meta| attrs.parse(meta));
    parse_macro_input!(args with parser);

    if attrs.spec.as_ref().is_none_or(|spec| spec.is_empty()) {
        return quote! {
            compile_error!("Missing required attribute `spec`");
        }
        .into();
    }

    if attrs.test.as_ref().is_none_or(|test| test.is_empty()) {
        return quote! {
            compile_error!("Missing required attribute `test`");
        }
        .into();
    }

    let spec = attrs.spec.unwrap();
    let test = attrs.test.unwrap();
    let main = quote_opt_str(&attrs.main);
    let max_samples = quote_opt_lit(&attrs.max_samples);
    let seed = quote_seed(&attrs.seed);

    let test_fn = parse_macro_input!(item as ItemFn);
    let test_attrs = test_fn.attrs;
    let test_ident = test_fn.sig.ident;
    let test_name = test_ident.to_string();
    let test_block = test_fn.block;

    quote! {
        #[test]
        #(#test_attrs)*
        fn #test_ident() -> anyhow::Result<()> {
            let driver = #test_block;
            let config = quint_connect::runner::Config {
                test_name: #test_name.to_string(),
                gen_config: quint_connect::runner::TestConfig {
                    spec: #spec.to_string(),
                    test: #test.to_string(),
                    main: #main,
                    max_samples: #max_samples,
                    seed: #seed.to_string(),
                }
            };
            quint_connect::runner::run_test(driver, config)
        }
    }
    .into()
}
