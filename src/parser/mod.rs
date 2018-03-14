#[cfg_attr(rustfmt, rustfmt_skip)]
#[allow(unknown_lints)]
#[allow(clippy)]
mod grammar {
    // During the build step, `build.rs` will output the generated parser to `OUT_DIR` to avoid
    // adding it to the source directory, so we just directly include the generated parser here.
    //
    // Even with `.gitignore` and the `exclude` in the `Cargo.toml`, the generated parser can still
    // end up in the source directory. This could happen when `cargo build` builds the file out of
    // the Cargo cache (`$HOME/.cargo/registrysrc`), and the build script would then put its output
    // in that cached source directory because of https://github.com/lalrpop/lalrpop/issues/280.
    // Later runs of `cargo vendor` then copy the source from that directory, including the 
    // generated file.
    include!(concat!(env!("OUT_DIR"), "/parser/grammar.rs"));
}

/// Contains all structures related to the AST for the WebIDL grammar.
pub mod ast;

/// Contains the visitor trait needed to traverse the AST and helper walk functions.
pub mod visitor;

pub use lalrpop_util::ParseError;

use lexer::{LexicalError, Token};

/// The result that is returned when an input string is parsed. If the parse succeeds, the `Ok`
/// result will be a vector of definitions representing the AST. If the parse fails, the `Err` will
/// be either an error from the lexer or the parser.
pub type ParseResult = Result<ast::AST, ParseError<usize, Token, LexicalError>>;

/// The parser that is used to parse WebIDL. It really serves as a wrapper around the parse
/// function exposed by lalrpop.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Parser;

impl Parser {
    /// Creates a new parser.
    pub fn new() -> Self {
        Parser
    }

    /// Parses a given input string and returns an AST.
    ///
    /// # Example
    ///
    /// ```
    /// use webidl::*;
    /// use webidl::ast::*;
    ///
    /// let parser = Parser::new();
    /// let result = parser.parse_string("[Attribute] interface Node { };");
    ///
    /// assert_eq!(result,
    ///            Ok(vec![Definition::Interface(Interface::NonPartial(NonPartialInterface {
    ///                 extended_attributes: vec![
    ///                     Box::new(ExtendedAttribute::NoArguments(
    ///                         Other::Identifier("Attribute".to_string())))],
    ///                 inherits: None,
    ///                 members: vec![],
    ///                 name: "Node".to_string()
    ///            }))]));
    /// ```
    pub fn parse_string(&self, input: &str) -> ParseResult {
        grammar::parse_Definitions(::Lexer::new(input))
    }
}
