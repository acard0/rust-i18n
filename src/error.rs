use std::fmt::Display;


#[derive(thiserror::Error, Debug)]
pub struct Error {
    source: Repr,
    details: ErrorDetails,
}

#[derive(thiserror::Error, Debug)]
pub struct Repr {
    source: Box<dyn std::error::Error>,
}

#[derive(Debug, Clone)]
pub struct ErrorDetails {
    pub display: String,
    pub name: String,
    pub fullname: String,
    pub suggestion_key: String,
    pub message: String,
    pub suggestion: Option<String>,
}

pub trait AsDetails {
    fn as_details(&self) -> ErrorDetails;
    fn get_message_key(&self) -> String;
    fn get_suggestion_key(&self) -> String;
    fn get_display_key(&self) -> String;
}

impl Error {
    pub fn new(source: impl std::error::Error + 'static, details: ErrorDetails) -> Self {
        Self { 
            source: Repr::new(source), 
            details
        }
    }

    pub fn get_details(&self) -> &ErrorDetails {
        &self.details
    }
    
    pub fn get_source(&self) -> &dyn std::error::Error {
        &*self.source.source
    }
}

impl Repr {
    pub fn new(source: impl std::error::Error + 'static) -> Self {
        Self { source: Box::new(source) }
    }
}

impl ErrorDetails {
    pub fn new(display: &str, name: &str, fullname: &str, suggestion_key: &str, message: &str, suggestion: Option<String>) -> Self {
        Self { display: display.to_owned(), name: name.to_owned(), fullname: fullname.to_owned(), suggestion_key: suggestion_key.to_owned(), message: message.to_string(), suggestion }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.get_details().message, f)
    }
}

impl Display for Repr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.source, f)
    }
}

impl Display for ErrorDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.message, f)
    }
}