use crate::ir2::{Add, BackLink, Link, MiddleNode, NodeType, RootNode, Scope};
use std::fmt::Debug;

pub trait IsParent: Clone + Into<Link<NodeType>> + Debug {
    fn get_children(&self) -> Link<Vec<Link<NodeType>>>;
    fn add_child(&mut self, mut child: Link<NodeType>) -> Link<NodeType> {
        self.get_children().borrow_mut().push(child.clone());
        child.swap_parent(self.clone().into());
        self.clone().into()
    }
    fn remove_child(&mut self, child: Link<NodeType>) -> Link<NodeType> {
        self.get_children().borrow_mut().retain(|c| c != &child);
        self.clone().into()
    }
    fn first(&self) -> Link<NodeType>
    where
        Self: Debug,
    {
        self.get_children()
            .borrow()
            .first()
            .expect("first() called on empty node")
            .clone()
    }
    fn last(&self) -> Link<NodeType>
    where
        Self: Debug,
    {
        self.get_children()
            .borrow()
            .last()
            .expect("last() called on empty node")
            .clone()
    }
    fn new_value<T>(&mut self, data: T) -> Link<NodeType>
    where
        T: Into<Link<NodeType>>,
    {
        let node: Link<NodeType> = data.into();
        self.add_child(node.clone());
        node
    }
    fn new_add(&mut self) -> Link<NodeType> {
        let node: Link<NodeType> = Add::default().into();
        self.add_child(node.clone());
        node
    }
    fn new_scope(&mut self) -> Link<NodeType> {
        let node: Link<NodeType> = Scope::default().into();
        self.add_child(node.clone());
        node
    }
}

pub trait IsChild: Clone + Into<Link<NodeType>> + Debug {
    fn get_parent(&self) -> BackLink<NodeType>;
    fn set_parent(&mut self, parent: Link<NodeType>);
    fn swap_parent(&mut self, new_parent: Link<NodeType>) {
        // Grab the old parent before we change it
        let old_parent = self.get_parent().to_link();
        // Remove self from the old parent's children
        if let Some(mut parent) = old_parent {
            if parent != new_parent {
                parent.remove_child(self.clone().into());
            }
        }
        // Change the parent
        self.set_parent(new_parent);
    }
}

trait NotParent {}
trait NotChild {}
trait IsNode: IsParent + IsChild {}
trait NotNode {}

impl<T: NotParent + IsChild> NotNode for T {}
impl<T: IsParent + IsChild> IsNode for T {}

impl NotChild for Graph {}
impl NotNode for Graph {}
impl<T> NotParent for Leaf<T> {}

#[derive(Clone, Eq, PartialEq)]
pub struct Node {
    parent: BackLink<NodeType>,
    children: Link<Vec<Link<NodeType>>>,
}

impl Node {
    pub fn new(parent: BackLink<NodeType>, children: Link<Vec<Link<NodeType>>>) -> Self {
        Self { parent, children }
    }
}

impl Default for Node {
    fn default() -> Self {
        Self {
            parent: BackLink::none(),
            children: Link::new(Vec::new()),
        }
    }
}

impl IsParent for Node {
    fn get_children(&self) -> Link<Vec<Link<NodeType>>> {
        self.children.clone()
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self.children)
    }
}

impl IsChild for Node {
    fn get_parent(&self) -> BackLink<NodeType> {
        self.parent.clone()
    }
    fn set_parent(&mut self, parent: Link<NodeType>) {
        self.parent = parent.into();
    }
}

impl From<Node> for Link<NodeType> {
    fn from(value: Node) -> Self {
        Link::new(NodeType::MiddleNode(MiddleNode::Scope(Scope::from(value))))
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct Leaf<T> {
    parent: BackLink<NodeType>,
    data: T,
}

impl<T> Leaf<T> {
    pub fn new(data: T) -> Self {
        Self {
            parent: BackLink::none(),
            data,
        }
    }
}

impl<T> Default for Leaf<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            parent: BackLink::none(),
            data: T::default(),
        }
    }
}

impl<T: Debug> Debug for Leaf<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self.data)
    }
}

impl<T: Clone + Debug> IsChild for Leaf<T>
where
    Leaf<T>: Into<Link<NodeType>>,
{
    fn get_parent(&self) -> BackLink<NodeType> {
        self.parent.clone()
    }
    fn set_parent(&mut self, parent: Link<NodeType>) {
        self.parent = parent.into();
    }
}

impl<T> From<Leaf<T>> for Link<NodeType>
where
    Leaf<T>: Into<NodeType>,
{
    fn from(value: Leaf<T>) -> Self {
        Link::new(value.into())
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct Graph {
    nodes: Link<Vec<Link<NodeType>>>,
}

impl Graph {
    pub fn create() -> Link<NodeType> {
        Graph::default().into()
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self {
            nodes: Link::new(Vec::default()),
        }
    }
}

impl IsParent for Graph {
    fn get_children(&self) -> Link<Vec<Link<NodeType>>> {
        self.nodes.clone()
    }
}

impl Debug for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Graph").field("nodes", &self.nodes).finish()
    }
}

impl From<Graph> for Link<NodeType> {
    fn from(value: Graph) -> Self {
        Link::new(NodeType::RootNode(RootNode::Graph(value)))
    }
}

#[cfg(test)]
mod tests {

    trait IsParentTest {
        fn parent(&self) -> bool {
            true
        }
    }
    trait NotParentTest {
        fn parent(&self) -> bool {
            false
        }
    }
    trait IsChildTest {
        fn child(&self) -> bool {
            true
        }
    }
    trait NotChildTest {
        fn child(&self) -> bool {
            false
        }
    }
    trait IsNodeTest {
        fn node(&self) -> bool {
            true
        }
    }
    trait NotNodeTest {
        fn node(&self) -> bool {
            false
        }
    }

    struct GraphTest;
    struct LeafTest;
    struct NodeTest;

    impl<T: NotParentTest + IsChildTest> NotNodeTest for T {}
    impl<T: IsParentTest + IsChildTest> IsNodeTest for T {}

    impl IsParentTest for GraphTest {}
    impl NotChildTest for GraphTest {}
    impl NotNodeTest for GraphTest {}
    impl NotParentTest for LeafTest {}
    impl IsChildTest for LeafTest {}
    impl IsParentTest for NodeTest {}
    impl IsChildTest for NodeTest {}

    #[test]
    fn test_negative_traits() {
        let graph = GraphTest;
        dbg!(graph.parent());
        dbg!(graph.child());
        dbg!(graph.node());
        assert!(graph.parent());
        assert!(!graph.child());
        assert!(!graph.node());

        let leaf = LeafTest;
        dbg!(leaf.parent());
        dbg!(leaf.child());
        dbg!(leaf.node());
        assert!(!leaf.parent());
        assert!(leaf.child());
        assert!(!leaf.node());

        let node = NodeTest;
        dbg!(node.parent());
        dbg!(node.child());
        dbg!(node.node());
        assert!(node.parent());
        assert!(node.child());
        assert!(node.node());
    }
}
