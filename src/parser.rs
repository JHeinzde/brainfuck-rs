struct ASTNode {
    child: Option<Box<ASTNode>>,
    body: Vec<Box<ASTNode>>,
    ty: NodeType,
}

impl ASTNode {
    pub fn set_child(mut self, n_child: Option<Box<ASTNode>>) {
        self.child = n_child;
    }

    pub fn add_to_body(mut self, new_content: Box<ASTNode>) {
        self.body.push(new_content);
    }
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

pub fn convert_source_to_op_chain(src_code: &str) -> Box<Box<ASTNode>> {
    let mut i: usize = 1;
    let mut in_while_body = false;
    let mut last_node: Box<ASTNode>;
    let mut while_loops: Vec<Box<ASTNode>> = Vec::new();
    let mut current_while_loop: usize = 0;

    last_node = Box::new(convert_char_to_op(src_code.chars().nth(0).unwrap()));
    let first_node = Box::new(last_node);


    while i != src_code.len() {
        let instruction = src_code
            .chars()
            .nth(i)
            .unwrap();
        let mut node = convert_char_to_op(instruction);

        if node.ty == NodeType::WhileEnd {
            current_while_loop -= 1;
            if current_while_loop == 0 {
                in_while_body = false;
            }
            continue;
        }

        let node = Box::new(node);
        last_node.child = Some(node);
        last_node = node;

        if in_while_body {
            while_loops.get(current_while_loop - 1)
                .unwrap()
                .add_to_body(node);
        }

        if node.ty == NodeType::WhileBegin {
            in_while_body = true;
            current_while_loop += 1;
            while_loops.push(node);
        }
        i += 1;
    }

    return first_node;
}

fn convert_char_to_op(chr: char) -> ASTNode {
    match chr {
        '>' => { ASTNode { child: None, body: vec![], ty: NodeType::PtrInc } }
        '<' => { ASTNode { child: None, body: vec![], ty: NodeType::PtrDec } }
        '+' => { ASTNode { child: None, body: vec![], ty: NodeType::CellInc } }
        '-' => { ASTNode { child: None, body: vec![], ty: NodeType::CellDec } }
        '.' => { ASTNode { child: None, body: vec![], ty: NodeType::PutChar } }
        ',' => { ASTNode { child: None, body: vec![], ty: NodeType::ReadChar } }
        '[' => { ASTNode { child: None, body: vec![], ty: NodeType::WhileBegin } }
        ']' => { ASTNode { child: None, body: vec![], ty: NodeType::WhileEnd } }
        _ => { ASTNode { child: None, body: vec![], ty: NodeType::Nop } }
    }
}

