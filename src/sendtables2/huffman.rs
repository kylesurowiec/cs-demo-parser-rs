#![allow(dead_code)]
#[derive(Clone)]
pub enum Node {
    Leaf {
        weight: i32,
        value: usize,
    },
    Branch {
        weight: i32,
        value: usize,
        left: Box<Node>,
        right: Box<Node>,
    },
}

impl Node {
    pub fn weight(&self) -> i32 {
        match self {
            | Node::Leaf { weight, .. } => *weight,
            | Node::Branch { weight, .. } => *weight,
        }
    }
    pub fn value(&self) -> usize {
        match self {
            | Node::Leaf { value, .. } => *value,
            | Node::Branch { value, .. } => *value,
        }
    }
    pub fn is_leaf(&self) -> bool {
        matches!(self, Node::Leaf { .. })
    }
    pub fn left(&self) -> &Node {
        match self {
            | Node::Branch { left, .. } => left,
            | _ => panic!("no left"),
        }
    }
    pub fn right(&self) -> &Node {
        match self {
            | Node::Branch { right, .. } => right,
            | _ => panic!("no right"),
        }
    }
}

#[allow(dead_code)]
pub fn build_huffman_tree(freqs: &[i32]) -> Node {
    use std::cmp::Ordering;
    use std::collections::BinaryHeap;

    struct HeapItem(i32, usize, Node);
    impl Ord for HeapItem {
        fn cmp(&self, other: &Self) -> Ordering {
            other.0.cmp(&self.0).then_with(|| other.1.cmp(&self.1))
        }
    }
    impl PartialOrd for HeapItem {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    impl PartialEq for HeapItem {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0 && self.1 == other.1
        }
    }
    impl Eq for HeapItem {}

    let mut heap: BinaryHeap<HeapItem> = BinaryHeap::new();
    for (i, &w) in freqs.iter().enumerate() {
        let w = if w == 0 { 1 } else { w };
        heap.push(HeapItem(
            w,
            i,
            Node::Leaf {
                weight: w,
                value: i,
            },
        ));
    }
    let mut next_val = freqs.len();
    while heap.len() > 1 {
        let a = heap.pop().unwrap();
        let b = heap.pop().unwrap();
        let node = Node::Branch {
            weight: a.0 + b.0,
            value: next_val,
            left: Box::new(a.2),
            right: Box::new(b.2),
        };
        next_val += 1;
        heap.push(HeapItem(node.weight(), node.value(), node));
    }
    heap.pop().unwrap().2
}
