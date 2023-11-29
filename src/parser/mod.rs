use crate::lexer::Token;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, char, multispace0, multispace1},
    combinator::{map, recognize},
    error::{context, convert_error, VerboseError},
    multi::separated_list0,
    sequence::{delimited, preceded, separated_pair, tuple},
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
pub enum AST {
    FunctionDef {
        name: String,
        params: Vec<(String, String)>,
        body: Box<AST>,
    },
    ReturnStatement(Box<AST>),
    BinaryExpr {
        left: Box<AST>,
        op: Token,
        right: Box<AST>,
    },
    Identifier(String),
}

type ParseResult<'a> = IResult<&'a str, AST, VerboseError<&'a str>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_param_list() {
        let input = "(a: int, b: int)";
        let g = parameter_list(input).unwrap();
        assert_eq!(
            g.1,
            vec![
                ("a".to_string(), "int".to_string()),
                ("b".to_string(), "int".to_string())
            ]
        );
    }

    #[test]
    fn test_binary_exp() {
        let input = "a + b";
        let g = binary_expr(input).unwrap();
        assert_eq!(
            g.1,
            AST::BinaryExpr {
                left: Box::new(AST::Identifier("a".to_string())),
                op: Token::Plus,
                right: Box::new(AST::Identifier("b".to_string())),
            },
        );
    }

    #[test]
    fn test_parse_function_def() {
        let input = "defpub something(a: int, b: int):\n    return a + b";
        match parse(input) {
            Ok(ast) => match ast {
                AST::FunctionDef { name, params, body } => {
                    assert_eq!(name, "something");
                    assert_eq!(params.len(), 2);
                    assert_eq!(params[0].0, "a");
                    assert_eq!(params[0].1, "int");
                    assert_eq!(params[1].0, "b");
                    assert_eq!(params[1].1, "int");
                    match *body {
                        AST::ReturnStatement(expr) => match *expr {
                            AST::BinaryExpr { left, op, right } => {
                                match *left {
                                    AST::Identifier(ref name) => assert_eq!(name, "a"),
                                    _ => panic!("Expected identifier for left operand"),
                                }
                                assert_eq!(op, Token::Plus);
                                match *right {
                                    AST::Identifier(ref name) => assert_eq!(name, "b"),
                                    _ => panic!("Expected identifier for right operand"),
                                }
                            }
                            _ => panic!("Expected binary expression in return statement"),
                        },
                        _ => panic!("Expected return statement"),
                    }
                }
                _ => panic!("Expected function definition"),
            },
            Err(e) => panic!("Failed to parse input: {:?}", e),
        }
    }
}

// Helper function to parse identifiers
fn identifier(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    recognize(alpha1)(input)
}
fn parameter(input: &str) -> IResult<&str, (String, String), VerboseError<&str>> {
    context(
        "parameter",
        map(
            tuple((identifier, multispace0, char(':'), multispace1, tag("int"))),
            |(name, _, _, _, _)| (name.to_string(), "int".to_string()),
        ),
    )(input)
}

fn parameter_list(input: &str) -> IResult<&str, Vec<(String, String)>, VerboseError<&str>> {
    delimited(
        char('('),
        separated_list0(
            preceded(multispace0, char(',')),
            preceded(multispace0, parameter),
        ),
        preceded(multispace0, char(')')),
    )(input)
}

// Helper function to parse a binary expression
fn binary_expr(input: &str) -> ParseResult {
    context(
        "binary exp",
        map(
            tuple((identifier, multispace0, char('+'), multispace0, identifier)),
            |(left, _, _, _, right)| AST::BinaryExpr {
                left: Box::new(AST::Identifier(left.to_string())),
                op: Token::Plus,
                right: Box::new(AST::Identifier(right.to_string())),
            },
        ),
    )(input)
}

// Helper function to parse a return statement
fn return_statement(input: &str) -> ParseResult {
    preceded(tuple((tag("return"), multispace1)), binary_expr)(input)
        .map(|(next_input, expr)| (next_input, AST::ReturnStatement(Box::new(expr))))
}

// The main parser for the function definition
fn function_def(input: &str) -> ParseResult {
    context(
        "function definition",
        map(
            tuple((
                tag("defpub"),
                multispace1,
                identifier,
                multispace0,
                parameter_list,
                tag(":"),
                multispace1,
                return_statement,
            )),
            |(_, _, name, _, params, _, _, ret_stmt)| AST::FunctionDef {
                name: name.to_string(),
                params: params
                    .into_iter()
                    .map(|(name, _)| (name.to_string(), "int".to_string()))
                    .collect(),
                body: Box::new(ret_stmt),
            },
        ),
    )(input)
}

// Entry point for the parser
pub fn parse(input: &str) -> Result<AST, String> {
    match function_def(input) {
        Ok((_remaining, ast)) => Ok(ast),
        Err(e) => Err(e.to_string()),
    }
}
