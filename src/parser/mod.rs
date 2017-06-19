#[cfg_attr(rustfmt, rustfmt_skip)]
mod grammar;

/// Contains all structures related to the AST for the WebIDL grammar.
pub mod ast;

use lalrpop_util::ParseError;

use lexer::{LexicalError, Token};

/// The result that is returned when an input string is parsed. If the parse succeeds, the `Ok`
/// result will be a vector of definitions representing the AST. If the parse fails, the `Err` will
/// be either an error from the lexer or the parser.
pub type ParseResult = Result<Vec<ast::Definition>, ParseError<usize, Token, LexicalError>>;

/// The parser that is used to parse WebIDL. It really serves as a wrapper around the parse
/// function exposed by lalrpop.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Parser;

impl Parser {
    /// Parses a given input string and returns an AST.
    ///
    /// # Example
    ///
    /// ```
    /// use webidl_parser::*;
    /// use webidl_parser::ast::*;
    ///
    /// let result = Parser::parse_string("[Attribute] interface Node { };");
    ///
    /// assert_eq!(result,
    ///            Ok(vec![Definition {
    ///                 definition_type: DefinitionType::Interface(Interface {
    ///                     members: vec![],
    ///                     name: "Node".to_string(),
    ///                     type_:
    ///                         InterfaceType::NonPartial(None),
    ///                 }),
    ///                 extended_attributes: vec![
    ///                     Box::new(ExtendedAttribute::Other {
    ///                         other: Other::Identifier("Attribute".to_string()),
    ///                         rest: None,
    ///                     })],
    ///            }]));
    ///
    /// ```
    pub fn parse_string(input: &str) -> ParseResult {
        grammar::parse_Definitions(::Lexer::new(input))
    }
}
