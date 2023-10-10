pub enum MariaDbVmError {
    Io { source: std::io::Error },
    Reqwest { source: reqwest::Error },
    UrlParseError { source: url::ParseError },
}

impl std::fmt::Display for MariaDbVmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io { source } => write!(f, "Could not load config: {}", source),
            Self::Reqwest { source } => write!(f, "Download error: {}", source),
            Self::UrlParseError { source } => write!(f, "URL parse error: {}", source),
        }
    }
}

impl std::fmt::Debug for MariaDbVmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io { source } => source.fmt(f),
            Self::Reqwest { source } => source.fmt(f),
            Self::UrlParseError { source } => source.fmt(f),
        }
    }
}

impl std::error::Error for MariaDbVmError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io { source } => Some(source),
            Self::Reqwest { source } => Some(source),
            Self::UrlParseError { source } => Some(source),
        }
    }
}

impl From<std::io::Error> for MariaDbVmError {
    fn from(source: std::io::Error) -> Self {
        Self::Io { source }
    }
}

impl From<reqwest::Error> for MariaDbVmError {
    fn from(source: reqwest::Error) -> Self {
        Self::Reqwest { source }
    }
}

impl From<url::ParseError> for MariaDbVmError {
    fn from(source: url::ParseError) -> Self {
        Self::UrlParseError { source }
    }
}
