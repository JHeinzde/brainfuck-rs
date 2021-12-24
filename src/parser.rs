use std::fmt::Error;
use crate::parser::ast::{Ast, NodeType};

mod ast;

pub fn sanitize_source(src_code: &mut String) {
    src_code.retain(|chr| {
        chr == '>' || chr == '<' || chr == '+' || chr == '-' || chr == '.' || chr == ',' || chr == '[' || chr == ']'
    })
}

pub fn convert_source_to_op_chain(src_code: &str) -> Ast {
    let mut ast = Ast::new();
    let mut i: usize = 0;
    let mut while_bodies = Vec::new();
    let mut current_while_loop: usize = 0;

    while i != src_code.len() {
        let n_type = convert_char_to_op(src_code.chars().nth(i)
            .unwrap());

        match n_type {
            NodeType::WhileBegin => {
                current_while_loop += 1;
                while_bodies.push(Ast::new());
                ast.push(n_type, None);
            }
            NodeType::WhileEnd => {
                current_while_loop -= 1;
                let while_body = while_bodies.pop()
                    .unwrap();
                if current_while_loop == 0 {
                    ast.set_last_body(Box::new(while_body));
                } else {
                    let last_index = while_bodies.len() - 1;
                    while_bodies.get_mut(last_index)
                        .unwrap()
                        .set_last_body(Box::new(while_body));
                }

                ast.push(n_type, None);
            }
            _ => {
                if current_while_loop == 0 {
                    ast.push(n_type, None);
                } else {
                    while_bodies.get_mut(current_while_loop - 1)
                        .unwrap()
                        .push(n_type, None)
                }
            }
        }
        i += 1;
    }

    return ast;
}

fn convert_char_to_op(chr: char) -> NodeType {
    match chr {
        '>' => NodeType::PtrInc,
        '<' => NodeType::PtrDec,
        '+' => NodeType::CellInc,
        '-' => NodeType::CellDec,
        '.' => NodeType::PutChar,
        ',' => NodeType::ReadChar,
        '[' => NodeType::WhileBegin,
        ']' => NodeType::WhileEnd,
        _ => NodeType::Nop // Should never occur
    }
}

