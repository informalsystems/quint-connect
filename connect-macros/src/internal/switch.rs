use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Expr, Ident, Token, Type, braced, parenthesized,
    parse::{Parse, ParseBuffer, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token::Paren,
};

struct Switch {
    step: Ident,
    cases: Vec<Case>,
}

impl Parse for Switch {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let step: Ident = input.parse()?;
        let mut cases: Vec<Case> = Vec::new();

        let actions;
        braced!(actions in input);

        while !actions.is_empty() {
            let case = actions.parse::<Case>()?;
            let is_block = matches!(case.handler, Some(Expr::Block(_)));
            cases.push(case);

            if !actions.is_empty() {
                if is_block {
                    let _ = actions.parse::<Token![,]>(); // optional comma
                } else {
                    actions.parse::<Token![,]>()?;
                }
            }
        }

        if cases.iter().filter(|case| case.action.is_none()).count() > 1 {
            return Err(input.error("switch! has more than one _ case"));
        }

        Ok(Self { step, cases })
    }
}

impl Switch {
    fn expand(self) -> proc_macro2::TokenStream {
        let Self { step, cases } = self;
        let cases: Vec<_> = cases.into_iter().map(Case::expand).collect();

        // Note that we allow unreachable code so that users can short-circuit
        // on action handlers without getting distracted by this warning.
        // Moreover, we reintroduce a binding named "step" so that it can be
        // reused later by other expansions in the file.
        //
        // For example:
        //     switch!(step { _ => todo!() })
        //
        // Expands to:
        //     let step = step; // <- make sure we have a binding named "step"
        //     match step.action_taken.as_str() {
        //         _ => {
        //             todo!();
        //             Status::Ok // <- unreachable code warning
        //         }
        //     }
        quote! {
            #[allow(unreachable_code)]
            {
                let step = #step;
                match step.action_taken.as_str() {
                    #(#cases),*
                    action => Status::Unimplemented(action.to_string())
                }
            }
        }
    }
}

struct Case {
    action: Option<Ident>,
    nondet_picks: Vec<NondetPick>,
    handler: Option<Expr>,
}

impl Parse for Case {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut action = None;
        let mut nondet_picks = Vec::new();
        let handler;

        if input.peek(Token![_]) {
            input.parse::<Token![_]>()?;
            handler = parse_handler(input)?;
            if handler.is_none() {
                return Err(input.error("_ case requires a handler block"));
            }
        } else {
            action = Some(input.parse()?);
            if input.peek(Paren) {
                let parens;
                parenthesized!(parens in input);
                nondet_picks.extend(Punctuated::<_, Token![,]>::parse_terminated(&parens)?);
            }
            handler = parse_handler(input)?;
        }

        Ok(Self {
            action,
            handler,
            nondet_picks,
        })
    }
}

fn parse_handler(input: &ParseBuffer<'_>) -> syn::Result<Option<Expr>> {
    if input.peek(Token![=>]) {
        input.parse::<Token![=>]>()?;
        return Ok(Some(input.parse()?));
    }
    Ok(None)
}

impl Case {
    fn expand(self) -> proc_macro2::TokenStream {
        let Self {
            action,
            nondet_picks,
            handler,
        } = self;

        match action {
            Some(action) => {
                let handler = handler.map(|expr| quote!(#expr)).unwrap_or_else(|| {
                    let nondets = nondet_picks.iter().map(|nondet| nondet.name.clone());
                    quote!(self.#action(#(#nondets),*))
                });
                let nondets = nondet_picks.into_iter().map(NondetPick::expand);
                let action_str = action.to_string();

                quote! {
                    #action_str => {
                        #(#nondets)*
                        #handler;
                        quint_connect::Status::Ok
                    }
                }
            }
            None => {
                assert!(handler.is_some(), "_ case requires a handler block");
                quote! {
                    _ => {
                        #handler;
                        quint_connect::Status::Ok
                    }
                }
            }
        }
    }
}

struct NondetPick {
    name: Ident,
    ty: Option<Type>,
    is_required: bool,
}

impl Parse for NondetPick {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        let mut ty = None;
        let mut is_required = true;

        if input.peek(Token![:]) {
            input.parse::<Token![:]>()?;
            ty = Some(input.parse()?);
        }

        if input.peek(Token![?]) {
            input.parse::<Token![?]>()?;
            is_required = false;
        }

        Ok(Self {
            name,
            ty,
            is_required,
        })
    }
}

impl NondetPick {
    fn expand(self) -> proc_macro2::TokenStream {
        let Self {
            name,
            ty,
            is_required,
        } = self;

        let name_str = name.to_string();
        let ty = ty.map(|ty| quote!(#ty)).unwrap_or_else(|| quote!(_));

        // TODO: avoid unwrap on these?
        if is_required {
            quote! {
                let #name = step.nondet_picks.get(#name_str);
                let #name: #ty = match #name {
                    Some(pick) => pick.try_into().unwrap(),
                    None => return quint_connect::Status::UnknownPick(#name_str.to_string()),
                };
            }
        } else {
            quote! {
                let #name: Option<#ty> = step
                    .nondet_picks
                    .get(#name_str)
                    .map(|nondet| nondet.try_into().unwrap());
            }
        }
    }
}

pub fn expand(input: TokenStream) -> TokenStream {
    let switch = parse_macro_input!(input as Switch);
    switch.expand().into()
}
