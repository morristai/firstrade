// Credit: https://github.com/apache/opendal/blob/main/core/src/types/error.rs

use std::backtrace::Backtrace;
use std::backtrace::BacktraceStatus;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ErrorKind {
    Unexpected,
    Unsupported,
    ConfigInvalid,
    NotFound,
    Unauthorized,
    Forbidden,
    ServerError,
    LoginFailed,
    RateLimited,
    ConditionNotMatch,
}

impl ErrorKind {
    pub fn into_static(self) -> &'static str {
        self.into()
    }

    /// Capturing a backtrace can be a quite expensive runtime operation.
    /// For some kinds of errors, backtrace is not useful and we can skip it (e.g., check if a file exists).
    ///
    /// See <https://github.com/apache/opendal/discussions/5569>
    fn disable_backtrace(&self) -> bool {
        matches!(self, ErrorKind::NotFound)
    }
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.into_static())
    }
}

impl From<ErrorKind> for &'static str {
    fn from(v: ErrorKind) -> &'static str {
        match v {
            ErrorKind::Unexpected => "Unexpected",
            ErrorKind::Unsupported => "Unsupported",
            ErrorKind::ConfigInvalid => "ConfigInvalid",
            ErrorKind::NotFound => "NotFound",
            ErrorKind::Unauthorized => "Unauthorized",
            ErrorKind::Forbidden => "Forbidden",
            ErrorKind::ServerError => "ServerError",
            ErrorKind::LoginFailed => "LoginFailed",
            ErrorKind::RateLimited => "RateLimited",
            ErrorKind::ConditionNotMatch => "ConditionNotMatch",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ErrorStatus {
    /// Permanent means without external changes, the error never changes.
    Permanent,
    Temporary,
    /// Persistent means this error used to be temporary but still failed after retry.
    Persistent,
}

impl Display for ErrorStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ErrorStatus::Permanent => write!(f, "permanent"),
            ErrorStatus::Temporary => write!(f, "temporary"),
            ErrorStatus::Persistent => write!(f, "persistent"),
        }
    }
}

/// Error is the error struct returned by all opendal functions.
///
/// ## Display
///
/// Error can be displayed in two ways:
///
/// - Via `Display`: like `err.to_string()` or `format!("{err}")`
///
/// Error will be printed in a single line:
///
/// ```shell
/// Unexpected, context: { path: /path/to/file, called: send_async } => something wrong happened, source: networking error"
/// ```
///
/// - Via `Debug`: like `format!("{err:?}")`
///
/// Error will be printed in multi lines with more details and backtraces (if captured):
///
/// ```shell
/// Unexpected => something wrong happened
///
/// Context:
///    path: /path/to/file
///    called: send_async
///
/// Source: networking error
///
/// Backtrace:
///    0: opendal::error::Error::new
///              at ./src/error.rs:197:24
///    1: opendal::error::tests::generate_error
///              at ./src/error.rs:241:9
///    2: opendal::error::tests::test_error_debug_with_backtrace::{{closure}}
///              at ./src/error.rs:305:41
///    ...
/// ```
///
/// - For conventional struct-style Debug representation, like `format!("{err:#?}")`:
///
/// ```shell
/// Error {
///     kind: Unexpected,
///     message: "something wrong happened",
///     status: Permanent,
///     operation: "Read",
///     context: [
///         (
///             "path",
///             "/path/to/file",
///         ),
///         (
///             "called",
///             "send_async",
///         ),
///     ],
///     source: Some(
///         "networking error",
///     ),
/// }
/// ```
pub struct Error {
    kind: ErrorKind,
    message: String,

    status: ErrorStatus,
    operation: &'static str,
    context: Vec<(&'static str, String)>,
    source: Option<anyhow::Error>,
    backtrace: Backtrace,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({}) at {}", self.kind, self.status, self.operation)?;

        if !self.context.is_empty() {
            write!(f, ", context: {{ ")?;
            write!(
                f,
                "{}",
                self.context
                    .iter()
                    .map(|(k, v)| format!("{k}: {v}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            )?;
            write!(f, " }}")?;
        }

        if !self.message.is_empty() {
            write!(f, " => {}", self.message)?;
        }

        if let Some(source) = &self.source {
            write!(f, ", source: {source}")?;
        }

        Ok(())
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // If alternate has been specified, we will print like Debug.
        if f.alternate() {
            let mut de = f.debug_struct("Error");
            de.field("kind", &self.kind);
            de.field("message", &self.message);
            de.field("status", &self.status);
            de.field("operation", &self.operation);
            de.field("context", &self.context);
            de.field("source", &self.source);
            return de.finish();
        }

        write!(f, "{} ({}) at {}", self.kind, self.status, self.operation)?;
        if !self.message.is_empty() {
            write!(f, " => {}", self.message)?;
        }
        writeln!(f)?;

        if !self.context.is_empty() {
            writeln!(f)?;
            writeln!(f, "Context:")?;
            for (k, v) in self.context.iter() {
                writeln!(f, "   {k}: {v}")?;
            }
        }
        if let Some(source) = &self.source {
            writeln!(f)?;
            writeln!(f, "Source:")?;
            writeln!(f, "   {source:#}")?;
        }
        if self.backtrace.status() == BacktraceStatus::Captured {
            writeln!(f)?;
            writeln!(f, "Backtrace:")?;
            writeln!(f, "{}", self.backtrace)?;
        }

        Ok(())
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|v| v.as_ref())
    }
}

impl Error {
    /// Create a new Error with error kind and message.
    pub fn new(kind: ErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),

            status: ErrorStatus::Permanent,
            operation: "",
            context: Vec::default(),
            source: None,
            // `Backtrace::capture()` will check if backtrace has been enabled
            // internally. It's zero cost if backtrace is disabled.
            backtrace: if kind.disable_backtrace() {
                Backtrace::disabled()
            } else {
                Backtrace::capture()
            },
        }
    }

    /// Update error's operation.
    ///
    /// # Notes
    ///
    /// If the error already carries an operation, we will push a new context
    /// `(called, operation)`.
    pub fn with_operation(mut self, operation: impl Into<&'static str>) -> Self {
        if !self.operation.is_empty() {
            self.context.push(("called", self.operation.to_string()));
        }

        self.operation = operation.into();
        self
    }

    /// Add more context in error.
    pub fn with_context(mut self, key: &'static str, value: impl ToString) -> Self {
        self.context.push((key, value.to_string()));
        self
    }

    /// Set source for error.
    ///
    /// # Notes
    ///
    /// If the source has been set, we will raise a panic here.
    pub fn set_source(mut self, src: impl Into<anyhow::Error>) -> Self {
        debug_assert!(self.source.is_none(), "the source error has been set");

        self.source = Some(src.into());
        self
    }

    /// Operate on error with map.
    pub fn map<F>(self, f: F) -> Self
    where
        F: FnOnce(Self) -> Self,
    {
        f(self)
    }

    /// Set permanent status for error.
    pub fn set_permanent(mut self) -> Self {
        self.status = ErrorStatus::Permanent;
        self
    }

    /// Set temporary status for error.
    ///
    /// By set temporary, we indicate this error is retryable.
    pub fn set_temporary(mut self) -> Self {
        self.status = ErrorStatus::Temporary;
        self
    }

    /// Set temporary status for error by given temporary.
    ///
    /// By set temporary, we indicate this error is retryable.
    #[allow(dead_code)]
    pub(crate) fn with_temporary(mut self, temporary: bool) -> Self {
        if temporary {
            self.status = ErrorStatus::Temporary;
        }
        self
    }

    /// Set persistent status for error.
    ///
    /// By setting persistent, we indicate the retry should be stopped.
    pub fn set_persistent(mut self) -> Self {
        self.status = ErrorStatus::Persistent;
        self
    }

    /// Return error's kind.
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }

    /// Check if this error is temporary.
    pub fn is_temporary(&self) -> bool {
        self.status == ErrorStatus::Temporary
    }
}

// TODO: modify test cases
#[cfg(test)]
mod tests {
    use anyhow::anyhow;
    use once_cell::sync::Lazy;
    use pretty_assertions::assert_eq;

    use super::*;

    static TEST_ERROR: Lazy<Error> = Lazy::new(|| Error {
        kind: ErrorKind::Unexpected,
        message: "something wrong happened".to_string(),
        status: ErrorStatus::Permanent,
        operation: "Read",
        context: vec![
            ("path", "/path/to/file".to_string()),
            ("called", "send_async".to_string()),
        ],
        source: Some(anyhow!("networking error")),
        backtrace: Backtrace::disabled(),
    });

    #[test]
    fn test_error_display() {
        let s = format!("{}", Lazy::force(&TEST_ERROR));
        assert_eq!(
            s,
            r#"Unexpected (permanent) at Read, context: { path: /path/to/file, called: send_async } => something wrong happened, source: networking error"#
        );
        println!("{:#?}", Lazy::force(&TEST_ERROR));
    }

    #[test]
    fn test_error_debug() {
        let s = format!("{:?}", Lazy::force(&TEST_ERROR));
        assert_eq!(
            s,
            r#"Unexpected (permanent) at Read => something wrong happened

Context:
   path: /path/to/file
   called: send_async

Source:
   networking error
"#
        )
    }
}
