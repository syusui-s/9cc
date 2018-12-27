use indoc::indoc;

use parser::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Assembly(String);

impl Assembly {
    fn new(value: &str) -> Assembly {
        Assembly(value.to_owned())
    }

    // ここをいい感じにしたい
    fn append(mut self, other: Assembly) -> Assembly {
        self.0.push_str(&other.0);

        self
    }

    pub fn value_ref<'a>(&'a self) -> &'a str {
        &self.0
    }
}

pub fn compile(program: Program) -> Assembly {
    Assembly::new(indoc!("
        .intel_syntax noprefix
        .global main
        main:
    "))
        .append(compile_program(program))
        .append(Assembly::new(indoc!("
        // return
            pop rax
            ret
        ")))
}

fn compile_program(program: Program) -> Assembly {
    match program {
        Program::Statement(statement) => compile_statement(statement),
        Program::Assignment(_) => panic!("Not Implemented"),
    }
}

fn compile_statement(statement: Statement) -> Assembly {
    match statement {
        Statement::Expr(expr) => compile_expr(expr),
    }
}

fn compile_expr(expr: Expr) -> Assembly {
    match expr {
        Expr::Add(multiply, e) =>
            compile_multiply(multiply)
                .append(compile_expr(*e))
                .append(Assembly::new(indoc!("
                // Add
                    pop rdi
                    pop rax
                    add rax, rdi
                    push rax
                "))),
        Expr::Sub(multiply, e) =>
            compile_multiply(multiply)
                .append(compile_expr(*e))
                .append(Assembly::new(indoc!("
                // Sub
                    pop rdi
                    pop rax
                    sub rax, rdi
                    push rax
                "))),
        Expr::Multiply(multiply) =>
            compile_multiply(multiply),
    }
}

fn compile_multiply(multiply: Multiply) -> Assembly {
    match multiply {
        Multiply::Mul(term, m) =>
            compile_term(term)
                .append(compile_multiply(*m))
                .append(Assembly::new(indoc!("
                // Mul
                    pop rdi
                    pop rax
                    mul rdi
                    push rax
                "))),
        Multiply::Div(term, m) =>
            compile_term(term)
                .append(compile_multiply(*m))
                .append(Assembly::new(indoc!("
                // Div
                    pop rdi
                    pop rax
                    mov rdx, 0
                    div rdi
                    push rax
                 "))),
        Multiply::Term(term) =>
            compile_term(term),
    }
}

fn compile_term(term: Term) -> Assembly {
    match term {
        Term::Int64(int64) =>
            Assembly(format!("    push {}\n", int64)),
    }
}
