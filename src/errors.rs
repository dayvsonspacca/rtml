#[derive(Debug)]
pub enum RtmlError {
    EmptyTemplate,
    TemplateNotFound,
}

impl core::fmt::Display for RtmlError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::EmptyTemplate => write!(f, "Template cannot be empty!"),
            Self::TemplateNotFound => write!(f, "Failed to read file at path!"),
        }
    }
}
