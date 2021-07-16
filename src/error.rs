pub type BoxError = std::boxed::Box<
    dyn std::error::Error // must implement Error to satisfy ?
        + std::marker::Send // needed for threads
        + std::marker::Sync, // needed for threads
>;

#[derive(Debug)]
pub struct Error {
    message: String,
    source: Option<BoxError>,
}

impl Error {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            source: None,
        }
    }

    pub fn boxed(self) -> BoxError {
        Box::new(self)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)?;
        if let Some(error) = &self.source {
            write!(f, "\nCaused by: {}", error)?;
        }
        Ok(())
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|error| error.as_ref() as &(dyn std::error::Error + 'static))
    }
}

unsafe impl Sync for Error {}
unsafe impl Send for Error {}
