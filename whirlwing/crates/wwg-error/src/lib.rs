use std::error::Error;

#[derive(Debug)]
pub struct WhirlwingError {
    content: String,
    kind: WhirlwingErrorKind,
    source: Option<Box<dyn Error>>,
}

impl WhirlwingError {
    pub fn new(content: String, kind: WhirlwingErrorKind) -> Self {
        WhirlwingError { content, kind, source: None, }
    }

    pub fn new_with_source(content: String, kind: WhirlwingErrorKind, source: Box<dyn Error>) -> Self {
        WhirlwingError { content, kind, source: Some(source), }
    }
}

impl std::fmt::Display for WhirlwingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.source.is_some() {
            write!(f, "Whirlwing Error: {}\nError Content: {}\nError Source: {}", self.kind, self.content, self.source.as_ref().unwrap())
        } else {
            write!(f, "Whirlwing Error: {}\nError Content: {}", self.kind, self.content)
        }
    }
}

impl std::error::Error for WhirlwingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_deref()
    }
}

#[derive(Debug)]
pub enum WhirlwingErrorKind {
    ShaderCompilationFailure,
}

impl std::fmt::Display for WhirlwingErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            WhirlwingErrorKind::ShaderCompilationFailure => "Shader Compilation Failure",
        };
        write!(f, "{output}")
    }
}