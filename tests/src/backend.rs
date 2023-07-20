use std::{collections::HashMap, sync::{Mutex, Arc}};

#[derive(Debug, Clone)]
pub struct I18n {
    inner: I18nHolder,
}

impl std::ops::Deref for I18n {
    type Target = I18nHolder;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl I18n {
    pub fn new() -> Self {
        Self { inner: I18nHolder::new() }
    }
}

impl rust_i18n::Backend for I18n {
    fn available_locales(&self) -> Vec<String> {
        self.trs.lock().unwrap().keys().cloned().collect()
    }

    fn translate(&self, locale: &str, key: &str) -> Option<String> {
        return self.trs.lock().unwrap().get(locale)?.get(key).cloned();
    }

    fn add(&mut self, locale: &str, key: &str, value: &str) {
        let mut trs = self.trs.lock().unwrap();
        let locale = trs.entry(locale.to_string())
            .or_insert_with(HashMap::new);

        locale.insert(key.to_string(), value.to_string());
    }
}

#[derive(Debug, Clone)]
pub struct I18nHolder {
    trs: Arc<Mutex<HashMap<String, HashMap<String, String>>>>,
}

impl I18nHolder {
    pub fn new() -> Self {
        Self { 
            trs: Arc::new(Mutex::new(HashMap::new()))
        }
    }
}