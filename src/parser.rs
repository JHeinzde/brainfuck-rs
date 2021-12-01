use std::ptr::null;

struct ASTNode<'a> {
    parent: Box<Option<&'a ASTNode<'a>>>,
    child: Box<Option<&'a ASTNode<'a>>>,
    body: Vec<Box<Option<&'a ASTNode<'a>>>>,
    n_type: NodeType,
}

#[derive(PartialEq)]
enum NodeType {
    PtrInc,
    PtrDec,
    CellInc,
    CellDec,
    PutChar,
    ReadChar,
    WhileBegin,
    WhileEnd,
    Nop,
}


pub fn sanitize_source(src_code: &mut String) {
    src_code.retain(|chr| {
        chr == '>' || chr == '<' || chr == '+' || chr == '-' || chr == '.' || chr == ',' || chr == '[' || chr == ']'
    })
}

pub fn convert_source_to_op_chain<'a>(src_code: &str) -> Box<ASTNode<'a>> {
    let mut i: usize = 1;
    let mut in_while_body = false;
    let mut last_node: ASTNode;

    let mut while_loops: Vec<Box<ASTNode>> = Vec::new();
    let mut current_while_loop: usize = 0;

    last_node = convert_char_to_op(src_code.chars().nth(0).unwrap());

    let first_node = Box::new(last_node);


    while i != src_code.len() {
        let instruction = src_code
            .chars()
            .nth(i)
            .unwrap();
        let mut node = convert_char_to_op(instruction);

        if node.n_type == NodeType::WhileEnd {
            current_while_loop -= 1;
            if current_while_loop == 0 {
                in_while_body = false;
            }
            continue;
        }

        node.parent = Box::new(Some(&last_node));
        last_node.child = Box::new(Some(&node));

        if in_while_body {
            while_loops.get(current_while_loop - 1)
                .unwrap()
                .body
                .push(Box::new(Some(&node)));
        }

        if node.n_type == NodeType::WhileBegin {
            in_while_body = true;
            current_while_loop += 1;
            while_loops.push(Box::new(node));
        }
    }

    return first_node;
}

fn convert_char_to_op<'a>(chr: char) -> ASTNode<'a> {
    match chr {
        '>' => { ASTNode { parent: Box::new(None), child: Box::new(None), body: vec![], n_type: NodeType::PtrInc } }
        '<' => { ASTNode { parent: Box::new(None), child: Box::new(None), body: vec![], n_type: NodeType::PtrDec } }
        '+' => { ASTNode { parent: Box::new(None), child: Box::new(None), body: vec![], n_type: NodeType::CellInc } }
        '-' => { ASTNode { parent: Box::new(None), child: Box::new(None), body: vec![], n_type: NodeType::CellDec } }
        '.' => { ASTNode { parent: Box::new(None), child: Box::new(None), body: vec![], n_type: NodeType::PutChar } }
        ',' => { ASTNode { parent: Box::new(None), child: Box::new(None), body: vec![], n_type: NodeType::ReadChar } }
        '[' => { ASTNode { parent: Box::new(None), child: Box::new(None), body: vec![], n_type: NodeType::WhileBegin } }
        ']' => { ASTNode { parent: Box::new(None), child: Box::new(None), body: vec![], n_type: NodeType::WhileEnd } }
        _ => { ASTNode { parent: Box::new(None), child: Box::new(None), body: vec![], n_type: NodeType::Nop } }
    }
}

