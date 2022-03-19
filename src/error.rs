use miette::{Diagnostic, SourceSpan};
use miette::{NamedSource};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
#[error("oops!")]
#[diagnostic(
    code(oops::my::bad),
    url(docsrs),
    help("try doing it better next time?")
)]
pub struct LoxCompileError {
    // The Source that we're gonna be printing snippets out of.
    // This can be a String if you don't have or care about file names.
    #[source_code]
    pub(crate) src: NamedSource,
    // Snippets and highlights can be included in the diagnostic!
    #[label("This bit here")]
    pub(crate) bad_bit: SourceSpan,
}

#[derive(Error, Debug, Diagnostic)]
#[error("oops!")]
#[diagnostic(
    code(oops::my::bad),
    url(docsrs),
    help("try doing it better next time?")
)]
pub struct LoxRuntimeError {}

impl LoxRuntimeError {
    pub fn new() -> Self {
        LoxRuntimeError {}
    }
}
