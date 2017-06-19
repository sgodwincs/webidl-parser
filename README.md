# webidl-parser

[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Build Status](https://travis-ci.org/sgodwincs/webidl-parser.svg?branch=master)](https://travis-ci.org/sgodwincs/webidl-parser)

A lexer and parser for [WebIDL](https://heycam.github.io/webidl/) in Rust.

# Example

## Lexing

```rust
use webidl_parser::*;

let lexer = Lexer::new("/* Example taken from emscripten site */\n\
                        enum EnumClass_EnumWithinClass {\n\
                            \"EnumClass::e_val\"\n\
                        };");
assert_eq!(lexer.collect::<Vec<_>>(),
           vec![Ok((41, Token::Enum, 45)),
                Ok((46, Token::Identifier("EnumClass_EnumWithinClass".to_string()), 71)),
                Ok((72, Token::LeftBrace, 73)),
                Ok((74, Token::StringLiteral("EnumClass::e_val".to_string()), 92)),
                Ok((93, Token::RightBrace, 94)),
                Ok((94, Token::Semicolon, 95))]);
```

## Parsing

```rust
use webidl_parser::*;
use webidl_parser::ast::*;

let result = Parser::parse_string("[Attribute] interface Node { };");

assert_eq!(result,
           Ok(vec![Definition {
                definition_type: DefinitionType::Interface(Interface {
                    members: vec![],
                    name: "Node".to_string(),
                    type_:
                        InterfaceType::NonPartial(None),
                }),
                extended_attributes: vec![
                    Box::new(ExtendedAttribute::Other {
                        other: Other::Identifier("Attribute".to_string()),
                        rest: None,
                    })],
           }]));
```
