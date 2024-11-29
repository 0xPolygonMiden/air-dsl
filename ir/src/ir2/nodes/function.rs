use crate::ir2::{IsNode, Node};

#[derive(Clone, Eq, PartialEq, Default, IsNode)]
pub struct Function {
    #[node(args, ret, body)]
    node: Node,
}
