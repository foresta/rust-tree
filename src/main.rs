use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: String,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: &str) -> Self {
        TreeNode {
            val: val.to_owned(),
            left: None,
            right: None,
        }
    }

    pub fn children(&self) -> Vec<Rc<RefCell<TreeNode>>> {
        [self.left.as_ref(), self.right.as_ref()]
            .iter()
            .flat_map(|x| *x)
            .map(|x| Rc::clone(x))
            .collect()
    }
}

fn bfs(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
    match root {
        Some(r) => _bfs_traverse(vec![r]),
        None => vec![],
    }
}

fn _bfs_traverse(node_list: Vec<Rc<RefCell<TreeNode>>>) -> Vec<String> {
    let next_list: Vec<Rc<RefCell<TreeNode>>> = node_list
        .iter()
        .flat_map(|x| x.borrow().children())
        .collect();

    let values = node_list
        .iter()
        .map(|x| x.borrow().to_owned().val)
        .collect();

    if next_list.is_empty() {
        values
    } else {
        let mut current = values;
        let mut children = _bfs_traverse(next_list);
        current.append(&mut children);
        current
    }
}

fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
    _dfs_traverse(root)
}

fn _dfs_traverse(node: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
    match node {
        None => vec![],
        Some(no) => {
            let n = no.borrow().to_owned();
            let mut current = vec![n.val];
            let mut left_children = _dfs_traverse(n.left);
            let mut right_children = _dfs_traverse(n.right);
            current.append(&mut left_children);
            current.append(&mut right_children);
            current
        }
    }
}

fn create_tree_node() -> Rc<RefCell<TreeNode>> {
    //
    //        A
    //    B       C
    //  D   E       F
    // G H   I     J K
    //

    let mut a = TreeNode::new("A");
    let mut b = TreeNode::new("B");
    let mut c = TreeNode::new("C");
    let mut d = TreeNode::new("D");
    let mut e = TreeNode::new("E");
    let mut f = TreeNode::new("F");
    let g = TreeNode::new("G");
    let h = TreeNode::new("H");
    let i = TreeNode::new("I");
    let j = TreeNode::new("J");
    let k = TreeNode::new("K");

    d.left = Some(Rc::new(RefCell::new(g)));
    d.right = Some(Rc::new(RefCell::new(h)));
    e.right = Some(Rc::new(RefCell::new(i)));
    f.left = Some(Rc::new(RefCell::new(j)));
    f.right = Some(Rc::new(RefCell::new(k)));

    b.left = Some(Rc::new(RefCell::new(d)));
    b.right = Some(Rc::new(RefCell::new(e)));
    c.right = Some(Rc::new(RefCell::new(f)));

    a.left = Some(Rc::new(RefCell::new(b)));
    a.right = Some(Rc::new(RefCell::new(c)));

    Rc::new(RefCell::new(a))
}

fn main() {
    println!("bfs: {:?}", bfs(Some(create_tree_node())));
    println!("dfs: {:?}", dfs(Some(create_tree_node())));
}
