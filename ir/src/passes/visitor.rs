use std::collections::HashSet;

use crate::{Node, NodeIndex, Operation};

pub trait Graph {
    fn children(&self, node: &Operation) -> Vec<NodeIndex>;
    fn node(&self, node_index: &NodeIndex) -> &Node;
}

pub enum VisitOrder {
    Manual,
    DepthFirst,
    PostOrder,
}
pub trait VisitDefault {}
pub trait VisitContext
where
    Self::Graph: Graph,
{
    type Graph;
    fn visit(&mut self, graph: &mut Self::Graph, node_index: NodeIndex);
    fn as_stack_mut(&mut self) -> &mut Vec<NodeIndex>;
    fn boundary_roots(&self, graph: &Self::Graph) -> HashSet<NodeIndex>;
    fn integrity_roots(&self, graph: &Self::Graph) -> HashSet<NodeIndex>;
    fn visit_order(&self) -> VisitOrder;
}
pub trait Visit: VisitContext {
    fn run(&mut self, graph: &mut Self::Graph) {
        match self.visit_order() {
            VisitOrder::Manual => self.visit_manual(graph),
            VisitOrder::PostOrder => self.visit_postorder(graph),
            VisitOrder::DepthFirst => self.visit_depthfirst(graph),
        }
        while let Some(node_index) = self.next_node() {
            self.visit(graph, node_index);
        }
    }
    fn visit_manual(&mut self, graph: &mut Self::Graph) {
        for root_index in self.boundary_roots(graph).iter().chain(self.integrity_roots(graph).iter()) {
            self.visit(graph, *root_index);
        }
    }
    fn visit_postorder(&mut self, graph: &mut Self::Graph) {
        for root_index in self.boundary_roots(graph).iter().chain(self.integrity_roots(graph).iter()) {
            self.visit_later(*root_index);
            let mut last: Option<NodeIndex> = None;
            while let Some(node_index) = self.peek() {
                let node = graph.node(&node_index);
                let children = graph.children(&node.op);
                if children.is_empty() || last.is_some() && children.contains(&last.unwrap()) {
                    self.visit(graph, node_index);
                    self.next_node();
                    last = Some(node_index);
                } else {
                    for child in children.iter().rev() {
                        self.visit_later(*child);
                    }
                }
            }
        }
    }
    fn visit_depthfirst(&mut self, graph: &mut Self::Graph) {
        for root_index in self.boundary_roots(graph).iter().chain(self.integrity_roots(graph).iter()) {
            self.visit_later(*root_index);
            while let Some(node_index) = self.next_node() {
                let node = graph.node(&node_index);
                let children = graph.children(&node.op);
                for child in children.iter().rev() {
                    self.visit_later(*child);
                }
                self.visit(graph, node_index);
            }
        }
    }
    fn peek(&mut self) -> Option<NodeIndex> {
        self.as_stack_mut().last().copied()
    }
    fn next_node(&mut self) -> Option<NodeIndex> {
        self.as_stack_mut().pop()
    }
    fn visit_later(&mut self, node_index: NodeIndex) {
        self.as_stack_mut().push(node_index);
    }
}

impl<T> Visit for T
where
    T: VisitContext + VisitDefault,
    T::Graph: Graph,
{
}
