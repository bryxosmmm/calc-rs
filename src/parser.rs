use std::{
    fs::File,
    io::{Result, Write},
};

use logos::{Lexer, Logos};
fn to_asm_op(lexer: &Lexer<Token>) -> String {
    match lexer.slice() {
        "+" => "add",
        "*" => "imul",
        "-" => "sub",
        "/" => "div",
        _ => "add",
    }
    .to_string()
}

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
enum Token {
    #[token(")")]
    RParen,
    #[token("(")]
    LParen,
    #[regex("[a-zA-Z]+")]
    Symbol,
    #[regex("[-*+/]", to_asm_op)]
    Operation(String),
    #[regex("[1-9][0-9]*", |lex| lex.slice().parse::<isize>().unwrap())]
    Number(isize),
}

#[derive(Debug)]
enum MathExpr {
    Number(isize),
    Epxr {
        a: Box<MathExpr>,
        b: Box<MathExpr>,
        op: String,
    },
}

fn expect_token(lex: &mut Lexer<Token>, token: Token) -> bool {
    lex.next().is_some_and(|t| token == t.unwrap())
}

fn get_token(lex: &mut Lexer<Token>) -> Token {
    lex.next().unwrap().unwrap()
}

fn _parse_expr(lex: &mut Lexer<Token>) -> MathExpr {
    match get_token(lex) {
        Token::Number(n) => MathExpr::Number(n),
        Token::LParen => parse_expr(lex),
        token => match token {
            Token::RParen => parse_expr(lex),
            Token::Operation(op) => {
                let a = parse_expr(lex);
                let b = parse_expr(lex);
                MathExpr::Epxr {
                    a: Box::new(a),
                    b: Box::new(b),
                    op,
                }
            }
            _ => panic!("Not expected token: {token:?}"),
        },
    }
}

fn parse_expr(lex: &mut Lexer<Token>) -> MathExpr {
    match get_token(lex) {
        Token::Number(n) => MathExpr::Number(n),
        Token::LParen => {
            let token = get_token(lex);
            match token {
                Token::RParen => MathExpr::Number(0),
                Token::Operation(op) => {
                    let a = parse_expr(lex);
                    let b = parse_expr(lex);
                    if !expect_token(lex, Token::RParen) {
                        panic!("in the end of expr need )");
                    }
                    MathExpr::Epxr {
                        a: Box::new(a),
                        b: Box::new(b),
                        op,
                    }
                }
                _ => panic!("Not expected token: {token:?}"),
            }
        }
        t => panic!("Main Node: Not expected token: {t:?}"),
    }
}

fn write_expr(f: &mut File, expr: &MathExpr) -> Result<()> {
    match expr {
        MathExpr::Number(n) => writeln!(f, "mov rax, {n}"),
        MathExpr::Epxr { a, b, op } => {
            write_expr(f, a)?;
            writeln!(f, "push rax")?;

            write_expr(f, b)?;
            writeln!(f, "pop rbx")?;
            if op == "div" {
                writeln!(f, "mov rdx, 0")?;
                writeln!(f, "mov rcx, rax")?; // b
                writeln!(f, "mov rax, rbx")?; // a
                writeln!(f, "div rcx")?;
            } else {
                writeln!(f, "{op} rax, rbx")?;
            }

            writeln!(f)
        }
    }
}

fn write(f: &mut File, expr: &MathExpr, fmt: String) -> Result<()> {
    writeln!(f, "format ELF64")?;
    writeln!(f, "section \".text\" executable")?;
    writeln!(f, "public main")?;
    writeln!(f, "extrn printf")?;
    writeln!(f, "msg db \"{fmt}\",10,0")?;
    writeln!(f, "main:")?;
    write_expr(f, expr)?;
    writeln!(f, "mov rdi, msg")?;
    writeln!(f, "mov rsi, rax")?;
    writeln!(f, "xor rax, rax")?;
    writeln!(f, "call printf")?;

    writeln!(f, "mov rax, 0")?;
    writeln!(f, "ret")?;
    Ok(())
}

pub fn parse_n_write(inp: &str, fmt: String, out: &str) -> Result<()> {
    let mut file = File::create(format!("./{out}.asm"))?;
    let mut lexer = Token::lexer(inp);
    let expr = parse_expr(&mut lexer);
    println!("DEBUG: {expr:?}");
    write(&mut file, &expr, fmt)?;
    Ok(())
}
