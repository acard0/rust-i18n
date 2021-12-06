#[macro_use]
extern crate rust_i18n;

i18n!("./locales");

pub fn hello() -> String {
    t!("hello")
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_t() {
        assert_eq!(t!("hello"), "Foo - Hello, World!");

        // Vars
        assert_eq!(
            t!("a.very.nested.message"),
            "Hello, %{name}. Your message is: %{msg}"
        );
        assert_eq!(
            t!("a.very.nested.message", name = "Jason"),
            "Hello, Jason. Your message is: %{msg}"
        );
        assert_eq!(
            t!("a.very.nested.message", name = "Jason", msg = "Bla bla"),
            "Hello, Jason. Your message is: Bla bla"
        );

        rust_i18n::set_locale("de");
        assert_eq!(t!("messages.hello", name = "world"), "Hallo, world!");

        rust_i18n::set_locale("en");
        assert_eq!(t!("messages.hello", name = "world"), "Hello, world!");
    }

    #[test]
    fn it_t_with_locale_and_args() {
        assert_eq!(t!("hello", locale = "de"), "Foo - Hallo Welt!");
        assert_eq!(t!("hello", locale = "en"), "Foo - Hello, World!");

        rust_i18n::set_locale("en");
        assert_eq!(t!("messages.hello", name = "Jason"), "Hello, Jason!");
        assert_eq!(
            t!("messages.hello", locale = "en", name = "Jason"),
            "Hello, Jason!"
        );
        assert_eq!(
            t!("messages.hello", locale = "de", name = "Jason"),
            "Hallo, Jason!"
        );
    }
}
