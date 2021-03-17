use std::collections::binary_heap::Iter;

enum Node<Item> {
    Leaf(Item),
    Children(Vec<Node<Item>>),
}

impl<It> Node<It> {
    fn traverse(&self, f: impl Fn(&It)) {
        match self {
            Node::Leaf(item) => f(item),
            Node::Children(children) => {
                children.iter().for_each(|node| node.traverse(&f));
            }
        }
    }

    fn iter(&self) -> NodeIter<'_, It> {
        NodeIter {
            children: std::slice::from_ref(self),
            parent: None,
        }
    }
}

struct NodeIter<'a, It> {
    children: &'a [Node<It>],
    parent: Option<Box<NodeIter<'a, It>>>,
}

impl<It> Default for NodeIter<'_, It> {
    fn default() -> Self {
        NodeIter {
            children: &[],
            parent: None,
        }
    }
}

impl<'a, It> Iterator for NodeIter<'a, It> {
    type Item = &'a It;

    fn next(&mut self) -> Option<Self::Item> {
        match self.children.get(0) {
            Some(Node::Leaf(item)) => {
                self.children = &self.children[1..];
                Some(item)
            }
            Some(Node::Children(children)) => {
                self.children = &self.children[1..];
                *self = NodeIter {
                    children: children.as_slice(),
                    parent: Some(Box::new(std::mem::take(self))),
                };
                self.next()
            }
            None => match self.parent.take() {
                Some(parent) => {
                    *self = *parent;
                    self.next()
                }
                None => None,
            },
        }
    }
}

impl<'a, It> IntoIterator for &'a Node<It> {
    type Item = &'a It;
    type IntoIter = NodeIter<'a, It>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

fn main() {
    let a = vec![0, 1, 2];
    let b: std::slice::Iter<i32> = a.iter();
}

#[test]
fn test_borrowing_iterator() {
    let tree = Node::Children(vec![
        Node::Leaf(5),
        Node::Leaf(4),
        Node::Children(vec![Node::Leaf(3), Node::Leaf(2), Node::Children(vec![])]),
        Node::Children(vec![Node::Children(vec![
            Node::Children(vec![Node::Leaf(1)]),
            Node::Leaf(0),
        ])]),
    ]);

    let nums: Vec<i32> = tree.iter().copied().collect();
    assert_eq!(nums, vec![5, 4, 3, 2, 1, 0]);
}

#[test]
fn test_borrowing_for_loop() {
    let tree = Node::Leaf(42);

    for &node in &tree {
        let _: i32 = node;
    }
}
