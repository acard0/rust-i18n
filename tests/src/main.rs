#![allow(dead_code, unused_variables)]

mod backend;

fn main() {

}

#[cfg(test)]
mod tests {
    use crate::backend::*;
    use rust_i18n::*;

    i18n!("tests/locales", fallback = "en-US", backend = I18n::new());

    #[test]
    fn test_load_all() {
        let map = load_locales("tests/locales", |_| false);
        for (locale, trs) in map {
            println!("{}: {:?}", locale, trs);
        }
    }
    
    #[test]
    fn basic() {
        t_add!("en", "hello", "Hello");
        t_add!("zh-CN", "foo", "Foo bar");
        t_add!("zh-CN", "hello", "你好");

        println!("locale: {}", rust_i18n::locale());

        rust_i18n::set_locale("en");
        
        assert_eq!(t!("hello"), "Hello");
        assert_eq!(t!("greetings"), "Greetings!");
        assert_eq!(t!("foo"), format!("{}.{}", rust_i18n::locale(), "foo"));
        assert_eq!(t!("foo", locale = "zh-CN"), "Foo bar");
        assert_eq!(t!("hello", locale = "zh-CN"), "你好");
             
        let t = t!("test_of", locale = "en", "a", "b");
        assert_eq!(t, "Test of a and b");

        let t = t!("test_of", locale = "tr-TR", "c", "d");
        assert_eq!(t, "c ve d testi");
        
        let t = t!("test_of", vec!["e", "f"]);
        assert_eq!(t, "Test of e and f");

        let w = vec!["g".to_owned(), "h".to_owned()];
        let t = t!("test_of", w);
        assert_eq!(t, "Test of g and h");

        rust_i18n::set_locale("tr-TR");
        assert_eq!(rust_i18n::locale(), "tr-TR".to_owned());

        let t = t!("test_of", locale = "en", "a", "b");
        assert_eq!(t, "Test of a and b");

        let t = t!("test_of", locale = "tr-TR", "c", "d");
        assert_eq!(t, "c ve d testi");
        
        let t = t!("test_of", vec!["e", "f"]);
        assert_eq!(t, "e ve f testi");

        let w = vec!["g".to_owned(), "h".to_owned()];
        let t = t!("test_of", w);
        assert_eq!(t, "g ve h testi");

        let a = "i";
        let t = t!("test_of", a, "j");
        assert_eq!(t, "i ve j testi");

        let t = t!("hellox", locale = "en", ["acar"]);
        assert_eq!(t, "Hello acar");

        let c = "k";
        let t = t!("hellox", [c]);
        assert_eq!(t, "Merhaba k");
    }

}