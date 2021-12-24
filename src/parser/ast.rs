use std::cmp::max;
use std::ops::DerefMut;

#[derive(Debug)]
pub struct Ast {
    head: Link,
}

pub struct IntoIter(Ast);

pub struct Iter<'a> {
    next: Option<&'a Node>,
}

pub struct IterMut<'a> {
    next: Option<&'a mut Node>,
}

#[derive(Debug)]
struct Node {
    next: Link,
    body: Option<Box<Ast>>,
    ty: NodeType,
}

pub struct AstNode {
    body: Option<Box<Ast>>,
    ty: NodeType,
}

type Link = Option<Box<Node>>;

#[derive(PartialEq, Debug)]
pub enum NodeType {
    PtrInc,
    PtrDec,
    CellInc,
    CellDec,
    PutChar,
    ReadChar,
    WhileBegin,
    WhileEnd,
    Nop, // should never exist but has to be there because of pattern matching
}


impl Ast {
    pub fn new() -> Self {
        Ast { head: None }
    }

    pub fn push(&mut self, ty: NodeType, body: Option<Box<Ast>>) {
        match &mut self.head {
            None => { self.head = Some(Box::new(Node::new(ty, body))) }
            Some(an) => {
                an.push(ty, body);
            }
        }
    }

    pub fn nth(&self, index: usize) -> (Option<&NodeType>, Option<&Box<Ast>>) {
        match &self.head {
            None => (None, None),
            Some(link) => link.nth(index)
        }
    }

    pub fn set_nth_body(&mut self, index: usize, body: Box<Ast>) {
        match &mut self.head {
            None => {}
            Some(link) => link.set_nth_body(index, body)
        }
    }

    pub fn set_last_body(&mut self, body: Box<Ast>) {
        match &mut self.head {
            None => {}
            Some(link) => link.set_last_body(body)
        }
    }

    pub fn pop(&mut self) -> Option<AstNode> {
        match &mut self.head {
            None => None,
            Some(_) => {
                let head = self.head.take();
                let head = head.unwrap();
                self.head = head.next;

                Some(AstNode {
                    ty: head.ty,
                    body: head.body,
                })
            }
        }
    }

    pub fn count_while_loops(&mut self) -> (usize, usize) {
        match &mut self.head {
            None => (0, 0),
            Some(head) => head.while_loop_visit()
        }
    }

    pub fn max_ptr_size(&mut self) -> usize {
        let (pinc_s, pdec_s) = self.ptr_size();
        max(pinc_s, pdec_s)
    }

    fn ptr_size(&mut self) -> (usize, usize) {
        match &mut self.head {
            None => (0, 0),
            Some(head) => {
                head.counter_ptr_ops()
            }
        }
    }

    pub fn into_iter(self) -> IntoIter {
        IntoIter(self)
    }

    pub fn iter(&mut self) -> Iter {
        Iter { next: self.head.as_deref() }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_> {
        IterMut { next: self.head.as_deref_mut() }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = (&'a NodeType, &'a Option<Box<Ast>>);

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            (&node.ty, &node.body)
        })
    }
}

impl Iterator for IntoIter {
    type Item = AstNode;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a> Iterator for IterMut<'a> {
    type Item = (&'a mut NodeType, &'a mut Option<Box<Ast>>);

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            (&mut node.ty, &mut node.body)
        })
    }
}

impl Node {
    fn new(ty: NodeType, body: Option<Box<Ast>>) -> Self {
        Node { next: None, body, ty }
    }

    fn push(&mut self, ty: NodeType, body: Option<Box<Ast>>) {
        match &mut self.next {
            None => { self.next = Some(Box::new(Node::new(ty, body))) }
            Some(next) => {
                next.push(ty, body);
            }
        }
    }

    fn nth(&self, index: usize) -> (Option<&NodeType>, Option<&Box<Ast>>) {
        if index == 0 {
            match &self.body {
                None => (Some(&self.ty), None),
                Some(ast) => (Some(&self.ty), Some(ast))
            }
        } else {
            match &self.next {
                None => (None, None),
                Some(next) => next.nth(index - 1)
            }
        }
    }

    fn set_nth_body(&mut self, index: usize, body: Box<Ast>) {
        if index == 0 {
            self.body = Some(body)
        } else {
            match &mut self.next {
                None => {}
                Some(next) => next.set_nth_body(index - 1, body)
            }
        }
    }

    fn set_last_body(&mut self, body: Box<Ast>) {
        match &mut self.next {
            None => {
                self.body = Some(body);
            }
            Some(next) => next.set_last_body(body)
        }
    }

    fn while_loop_visit(&mut self) -> (usize, usize) {
        let mut result = (0, 0);
        match &mut self.ty {
            NodeType::WhileBegin => result.0 = result.0 + 1,
            NodeType::WhileEnd => result.1 = result.1 + 1,
            _ => {}
        }

        match &mut self.body {
            None => {}
            Some(body) => {
                let (wb_result, we_result) = body.count_while_loops();
                result.0 += wb_result;
                result.1 += we_result;
            }
        }

        match &mut self.next {
            None => {}
            Some(next) => {
                let (wb_next, we_next) = next.while_loop_visit();
                result.0 += wb_next;
                result.1 += we_next;
            }
        }

        return result;
    }

    fn counter_ptr_ops(&mut self) -> (usize, usize) {
        let mut result = (0, 0);
        match &mut self.ty {
            NodeType::PtrInc => result.0 = result.0 + 1,
            NodeType::PtrDec => result.1 = result.1 + 1,
            _ => {}
        }

        match &mut self.body {
            None => {}
            Some(body) => {
                let (pinc_body, pdec_body) = body.ptr_size();
                result.0 += pinc_body;
                result.1 += pdec_body;
            }
        }

        match &mut self.next {
            None => {}
            Some(next) => {
                let (pinc_next, pdec_body) = next.counter_ptr_ops();
                result.0 += pinc_next;
                result.1 += pdec_body;
            }
        }

        return result;
    }
}


#[cfg(test)]
mod test {
    use crate::parser::ast::{Ast, NodeType};

    #[test]
    fn basic_test() {
        let mut ast = Ast::new();
        ast.push(NodeType::CellInc, None);
        ast.push(NodeType::WhileBegin, None);
        ast.push(NodeType::WhileEnd, None);


        let mut while_body = Ast::new();
        while_body.push(NodeType::CellDec, None);
        ast.set_nth_body(1, Box::new(while_body));

        let node = ast.pop();
        let node = node.unwrap();

        assert_eq!(NodeType::CellInc, node.ty);
        assert!(node.body.is_none());
    }

    #[test]
    fn into_iter_test() {
        let mut ast = Ast::new();
        ast.push(NodeType::CellInc, None);
        ast.push(NodeType::WhileBegin, None);
        ast.push(NodeType::WhileEnd, None);

        let mut body = Ast::new();
        body.push(NodeType::CellInc, None);
        ast.set_nth_body(2, Box::new(body));

        let mut iterator = ast.into_iter();
        let item = iterator.next();
        assert!(item.is_some());
        let node = item.unwrap();

        assert!(node.body.is_none());
        assert_eq!(node.ty, NodeType::CellInc);
    }

    #[test]
    fn iter_test() {
        let mut ast = Ast::new();
        ast.push(NodeType::CellInc, None);
        ast.push(NodeType::WhileBegin, None);
        ast.push(NodeType::WhileEnd, None);

        let mut iter = ast.iter();

        let node = iter.next();
        assert!(node.is_some());

        let (n_type, body) = node.unwrap();
        assert_eq!(&NodeType::CellInc, n_type);
        assert!(body.is_none());
        let (n_type, body) = iter.next().unwrap();
        assert_eq!(&NodeType::WhileBegin, n_type);
        assert!(body.is_none());
        let (n_type, body) = iter.next().unwrap();
        assert_eq!(&NodeType::WhileEnd, n_type);
        assert!(body.is_none());

        let opt = iter.next();
        assert!(opt.is_none());
    }

    #[test]
    fn test_while_loop_counter() {
        let mut ast = Ast::new();
        ast.push(NodeType::CellInc, None);
        ast.push(NodeType::WhileBegin, None);
        ast.push(NodeType::WhileEnd, None);

        let mut ast_two = Ast::new();
        ast_two.push(NodeType::WhileBegin, None);
        ast_two.push(NodeType::CellInc, None);

        ast.set_nth_body(1, Box::new(ast_two));

        assert_eq!((2, 1), ast.count_while_loops());
    }

    #[test]
    fn test_ptr_counter() {
        let mut ast = Ast::new();
        ast.push(NodeType::PtrInc, None);
        ast.push(NodeType::PtrInc, None);
        ast.push(NodeType::PtrDec, None);

        ast.push(NodeType::WhileBegin, None);
        ast.push(NodeType::CellInc, None);

        assert_eq!(2, ast.max_ptr_size());
    }
}
