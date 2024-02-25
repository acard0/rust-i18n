#![allow(rustdoc::invalid_rust_codeblocks)]
#![doc = include_str!("../README.md")]

use once_cell::sync::Lazy;
use std::sync::Mutex;

pub mod error;

#[doc(hidden)]
pub use once_cell;
pub use rust_i18n_macros::*;
pub use rust_i18n_support::*;
pub use rust_i18n_support::backend::*;

static CURRENT_LOCALE: Lazy<Mutex<String>> = Lazy::new(|| {
    get_locale()
        .map(|locale| Mutex::new(locale.to_string()))
        .unwrap_or_else(|| Mutex::new("en-US".to_string()))
});

/// Set current locale
pub fn set_locale(locale: &str) {
    let mut current_locale = CURRENT_LOCALE.lock().unwrap();
    *current_locale = locale.to_string();
}

/// Get current locale
pub fn locale() -> String {
    CURRENT_LOCALE.lock().unwrap().to_string()
}

pub fn fmt<I, S>(s: &str, vals: I) -> String
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut res = s.to_string();
    for (i, val) in vals.into_iter().enumerate() {
        let placeholder = format!("{{{}}}", i);
        res = res.replace(&placeholder, val.as_ref());
    }

    res
}

/// Get I18n text
///
/// ```ignore
/// // Simple get text with current locale
/// t!("greeting"); // greeting: "Hello world" => "Hello world"
/// // Get a special locale's text
/// t!("greeting", locale = "de"); // greeting: "Hallo Welt!" => "Hallo Welt!"
///
/// // With variables
/// t!("messages.hello", "world"); // messages.hello: "Hello, {}" => "Hello, world"
/// t!("messages.foo", "Foo", "Bar"); // messages.foo: "Hello, {} and {}" => "Hello, Foo and Bar"
///
/// // With locale and variables
/// t!("messages.hello", locale = "de", "Jason"); // messages.hello: "Hallo, {}" => "Hallo, Jason"
/// ```
#[macro_export]
#[allow(clippy::crate_in_macro_def)]
macro_rules! t {
    // t!("foo")
    ($key:expr) => {
        _rust_i18n_translate(rust_i18n::locale().as_str(), $key)
    };

    // t!("foo", locale = "en", vec!["bar", "baz"])
    ($key:expr, locale = $locale:expr, $vals:expr) => {{
        let mut message = _rust_i18n_translate($locale, $key);
        rust_i18n::fmt(&message, $vals)
    }};

    // t!("foo", locale = "en", "bar", "baz")
    ($key:expr, locale = $locale:expr, $( $x:expr ),*) => {{
        let mut message = _rust_i18n_translate($locale, $key);
        let vals: Vec<&str> = vec![$($x),*];
        rust_i18n::fmt(&message, &vals)
    }};

    // t!("foo", locale = "en")
    ($key:expr, locale = $locale:expr) => {
        _rust_i18n_translate($locale, $key)
    };

    // t!("foo", locale = "en", a = 1, b = "Foo")
    ($key:expr, locale = $locale:expr, $($var_name:tt = $var_val:expr),+ $(,)?) => {
        {
            let mut message = _rust_i18n_translate($locale, $key);

            $(
                // Get the variable name as a string, and remove quotes surrounding the variable name
                let var_name = stringify!($var_name).trim_matches('"');
                // Make a holder string to replace the variable name with: %{var_name}
                let holder = format!("%{{{var_name}}}");

                message = message.replace(&holder, &format!("{}", $var_val));
            )+
            message
        }
    };

    // t!("foo %{a} %{b}", a = "bar", b = "baz")
    ($key:expr, $($var_name:tt = $var_val:expr),+ $(,)?) => {
        {
            rust_i18n::t!($key, locale = &rust_i18n::locale(), $($var_name = $var_val),*)
        }
    };

    // t!("foo %{a} %{b}", locale = "en", "a" => "bar", "b" => "baz")
    ($key:expr, locale = $locale:expr, $($var_name:tt => $var_val:expr),+ $(,)?) => {
        {
            rust_i18n::t!($key, locale = $locale, $($var_name = $var_val),*)
        }
    };

    // t!("foo %{a} %{b}", "a" => "bar", "b" => "baz")
    ($key:expr, $($var_name:tt => $var_val:expr),+ $(,)?) => {
        {
            rust_i18n::t!($key, locale = &rust_i18n::locale(), $($var_name = $var_val),*)
        }
    };

    // t!("foo", vec!["bar", "baz"])
    ($key:expr, $vals:expr) => {{
        rust_i18n::t!(
            $key,
            locale = rust_i18n::locale().as_str(),
            $vals
        )
    }};
    
    // t!("foo", "bar", "baz")
    ($key:expr, $( $x:expr ),*) => {{
        rust_i18n::t!(
            $key,
            locale = rust_i18n::locale().as_str(),
            vec![$($x),*]
        )
    }};
}

/// Get available locales
///
/// ```ignore
/// rust_i18n::available_locales!();
/// // => ["en", "zh-CN"]
/// ```
#[macro_export(local_inner_macros)]
#[allow(clippy::crate_in_macro_def)]
macro_rules! available_locales {
    () => {
        _rust_i18n_available_locales()
    };
}


#[macro_export]
#[allow(clippy::crate_in_macro_def)]
macro_rules! t_add {
    // t_add!("en", "messages.welcome", "Welcome %{name}")
    ($locale:expr, $key:expr, $value:expr) => {
        _rust_i18n_add($locale, $key, $value)
    };

    // t_add!("messages.welcome", "Welcome %{name}")
    ($key:expr, $value:expr) => {
        _rust_i18n_add(rust_i18n::locale().as_str(), $key, $value)
    };
}
