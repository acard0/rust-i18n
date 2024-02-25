use quote::quote;
use rust_i18n_support::{is_debug, load_locales};
use std::collections::HashMap;
use syn::{parse_macro_input, DeriveInput, Expr, Ident, LitStr, Token};

struct Args {
    locales_path: String,
    fallback: Option<String>,
    extend: Option<Expr>,
}

impl Args {
    fn consume_path(&mut self, input: syn::parse::ParseStream) -> syn::parse::Result<()> {
        let path = input.parse::<LitStr>()?;
        self.locales_path = path.value();

        Ok(())
    }

    fn consume_options(&mut self, input: syn::parse::ParseStream) -> syn::parse::Result<()> {
        let ident = input.parse::<Ident>()?.to_string();
        input.parse::<Token![=]>()?;

        match ident.as_str() {
            "fallback" => {
                let val = input.parse::<LitStr>()?.value();
                self.fallback = Some(val);
            }
            "backend" => {
                let val = input.parse::<Expr>()?;
                self.extend = Some(val);
            }
            _ => {}
        }

        // Continue to consume reset of options
        if input.parse::<Token![,]>().is_ok() {
            self.consume_options(input)?;
        }

        Ok(())
    }
}

impl syn::parse::Parse for Args {
    /// Parse macro arguments.
    ///
    /// ```ignore
    /// i18n!();
    /// i18n!("locales");
    /// i18n!("locales", fallback = "en");
    /// ```
    ///
    /// Ref: https://docs.rs/syn/latest/syn/parse/index.html
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let lookahead = input.lookahead1();

        let mut result = Self {
            locales_path: String::from("locales"),
            fallback: None,
            extend: None,
        };

        if lookahead.peek(LitStr) {
            result.consume_path(input)?;

            if input.parse::<Token![,]>().is_ok() {
                result.consume_options(input)?;
            }
        } else if lookahead.peek(Ident) {
            result.consume_options(input)?;
        }

        Ok(result)
    }
}

/// Init I18n translations.
///
/// This will load all translations by glob `**/*.yml` from the given path, default: `${CARGO_MANIFEST_DIR}/locales`.
///
/// Attribute `fallback` for set the fallback locale, if present `t` macro will use it as the fallback locale.
///
/// ```ignore
/// i18n!();
/// i18n!("locales");
/// i18n!("locales", fallback = "en");
/// ```
#[proc_macro]
pub fn i18n(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let args = parse_macro_input!(input as Args);

    // CARGO_MANIFEST_DIR is current build directory
    let cargo_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is empty");
    let current_dir = std::path::PathBuf::from(cargo_dir);
    let locales_path = current_dir.join(&args.locales_path);

    let data = load_locales(&locales_path.display().to_string(), |_| false);
    let code = generate_code(data, args);

    if is_debug() {
        println!(
            "\n\n-------------- code --------------\n{}\n----------------------------------\n\n",
            code
        );
    }

    code.into()
}

fn generate_code(
    translations: HashMap<String, HashMap<String, String>>,
    args: Args,
) -> proc_macro2::TokenStream {
    let mut all_translations = Vec::<proc_macro2::TokenStream>::new();

    translations.iter().for_each(|(locale, trs)| {
        let mut sub_trs = Vec::<proc_macro2::TokenStream>::new();

        trs.iter().for_each(|(k, v)| {
            let k = k.to_string();
            let v = v.to_string();
            sub_trs.push(quote! {
                (#k, #v)
            });
        });

        all_translations.push(quote! {
            let trs = [#(#sub_trs),*];
            backend.add_translations(#locale, &trs.into_iter().collect());
        });
    });

    let fallback = if let Some(fallback) = args.fallback {
        quote! {
            Some(#fallback)
        }
    } else {
        quote! {
            None
        }
    };

    let extend_code = if let Some(extend) = args.extend {
        quote! {
            let backend = backend.extend(#extend);
        }
    } else {
        quote! {}
    };

    // result
    quote! {
        use rust_i18n::BackendExt;
        use std::sync::{Arc, Mutex};

        /// I18n backend instance
        static _RUST_I18N_BACKEND: rust_i18n::once_cell::sync::Lazy<Arc<Mutex<Box<dyn rust_i18n::Backend>>>> = rust_i18n::once_cell::sync::Lazy::new(|| {
            let mut backend = rust_i18n::SimpleBackend::new();
            #(#all_translations)*
            #extend_code

            Arc::new(Mutex::new(Box::new(backend)))
        });

        static _RUST_I18N_FALLBACK_LOCALE: Option<&'static str> = #fallback;

        /// Get I18n text by locale and key
        #[inline]
        pub fn _rust_i18n_translate(locale: &str, key: &str) -> String {
            let mut backend = _RUST_I18N_BACKEND.lock().unwrap();

            if let Some(value) = backend.translate(locale, key) {
                return value.to_string();
            }

            if let Some(fallback) = _RUST_I18N_FALLBACK_LOCALE {
                if let Some(value) = backend.translate(fallback, key) {
                    return value.to_string();
                }
            }

            key.to_owned()
        }
    
        #[inline]
        pub fn _rust_i18n_add(locale: &str, key: &str, value: &str) {
            let mut backend = _RUST_I18N_BACKEND.lock().unwrap();
            backend.add(locale, key, value);
        }

        pub fn _rust_i18n_available_locales() -> Vec<String> {
            let mut backend = _RUST_I18N_BACKEND.lock().unwrap();
            let mut locales = backend.available_locales();
            locales.sort();
            locales
        }
    }
}

#[proc_macro_derive(AsDetails)]
pub fn derive_this_error(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;

    let impl_block = quote! {
        impl AsDetails for #name {
            fn get_message_key(&self) -> String {
                use convert_case::{Case, Casing};
                let name = stringify!(#name).to_case(Case::Kebab);
                let inner = self.to_string();
                format!("{}.{}", &name, &inner)
            }

            fn get_suggestion_key(&self) -> String {
                format!("{}.suggestion", self.get_message_key())
            }
            
            fn as_details(&self) -> rust_i18n_support::error::ErrorDetails {
                use convert_case::{Case, Casing};
                let name = stringify!(#name).to_case(Case::Kebab);
                let message_key = self.get_message_key();
                let suggestion_key = self.get_suggestion_key();
        
                let message = t!(&message_key);
                let suggestion = t!(&suggestion_key);
        
                let suggestion = match suggestion != suggestion_key {
                    true => Some(suggestion),
                    false => None
                };

                rust_i18n_support::error::ErrorDetails::new(&name, &message, suggestion)
            }
        }

        impl From<#name> for Error {
            fn from(value: #name) -> Self {
                use convert_case::*;
                let details = value.as_details();
                rust_i18n_support::error::Error::new(value, details)
            }
        }
    };

    proc_macro::TokenStream::from(impl_block)
}