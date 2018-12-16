use indoc::indoc;

use parser::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Assembly(String);

impl Assembly {
    fn append(self, other: Assembly) -> Assembly {
        Assembly(self.0 + &other.0)
    }
}

pub fn compile(program: Program) -> Assembly {
    compile_program(program)
}

fn compile_program(program: Program) -> Assembly {
    match program {
        Program::Expr(expr) => compile_expr(expr),
    }
}

fn compile_expr(expr: Expr) -> Assembly {
    match expr {
        Expr::Add(multiply, e) =>
            compile_multiply(multiply)
                .append(compile_expr(*e)),
        Expr::Sub(multiply, e) =>
            compile_multiply(multiply)
                .append(compile_expr(*e)),
        Expr::Multiply(multiply) =>
            compile_multiply(multiply),
    }
}

fn compile_multiply(multiply: Multiply) -> Assembly {
    match multiply {
        Multiply::Mul(term, m) =>
            compile_term(term)
                .append(compile_multiply(*m)),
        Multiply::Div(term, m) =>
            compile_term(term)
                .append(compile_multiply(*m))
                .append(Assembly(indoc!("
                    pop rdi
                    pop rax
                    add rax, rdi
                    push rax
                 ").to_owned())),
        Multiply::Term(term) =>
            compile_term(term),
    }
}

fn compile_term(term: Term) -> Assembly {
    match term {
        Term::Int64(int64) =>
            Assembly(format!("  push {}", int64)),
    }
}
