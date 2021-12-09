use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::ops::Deref;
use std::ptr::null;
use std::rc::Rc;

struct ASTNode {
    child: Option<Rc<RefCell<ASTNode>>>,
    body: Vec<Rc<RefCell<ASTNode>>>,
    ty: NodeType,
}

impl ASTNode {
    pub fn set_child(mut self, n_child: Option<Rc<RefCell<ASTNode>>>) {
        self.child = n_child;
    }

    pub fn add_to_body(mut self, new_content: Rc<RefCell<ASTNode>>) {
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

pub fn convert_source_to_op_chain(src_code: &str) -> Rc<RefCell<ASTNode>> {
    let mut i: usize = 1;
    let mut in_while_body = false;
    let mut last_node: Rc<RefCell<ASTNode>>;

    let mut while_loops: Vec<Rc<RefCell<ASTNode>>> = Vec::new();
    let mut current_while_loop: usize = 0;

    last_node = Rc::new(RefCell::new(convert_char_to_op(src_code.chars().nth(0).unwrap())));

    let first_node = Rc::clone(&last_node);


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

        let node = Rc::new(RefCell::new(node));

        last_node.deref().into_inner().set_child(Some(Rc::clone(&node)));

        if in_while_body {
            while_loops.get(current_while_loop - 1)
                .unwrap()
                .into_inner()
                .add_to_body(Rc::clone(&node));
        }

        if node.into_inner().ty == NodeType::WhileBegin {
            in_while_body = true;
            current_while_loop += 1;
            while_loops.push(Rc::clone(&node));
        }
    }

    return first_node;
}

fn convert_char_to_op<'a>(chr: char) -> ASTNode {
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

