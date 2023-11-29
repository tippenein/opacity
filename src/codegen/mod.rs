use crate::lexer::Token;
use crate::parser::AST;

pub fn generate_code(ast: AST) -> String {
    match ast {
        AST::FunctionDef { name, params, body } => {
            let mut code = format!("(define-private ({} ", name);
            for (param_name, _param_type) in params {
                // Assuming all parameters are of type 'int' and map to 'uint' in the target language
                code.push_str(&format!("({} uint) ", param_name));
            }
            code.push_str(") ");
            code.push_str(&generate_code(*body));
            code.push(')');
            code
        }
        AST::ReturnStatement(expr) => {
            format!("(+ {})", generate_code(*expr))
        }
        AST::BinaryExpr { left, op, right } => match op {
            Token::Plus => {
                format!("(+ {} {})", generate_code(*left), generate_code(*right))
            }
            _ => unimplemented!("Code generation for this operator is not implemented"),
        },
        AST::Identifier(name) => name,
    }
}
