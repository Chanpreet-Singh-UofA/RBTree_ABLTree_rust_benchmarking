use std::cell::RefCell;
use std::rc::{Rc, Weak};
use core::cmp::max;
use std::fmt::{self, Debug};
use std::marker::Copy;
use std::num;
use std::io::stdin;
use std::io;
use std::io::stdout;
use std::io::Write;
use std::process;



#[derive(Debug, PartialEq, Clone, Copy)]
pub enum NodeColor {
    Red,
    Black,
}
#[derive(Debug, Clone)]
pub struct TreeNode<T>  {
    pub color: NodeColor,
    pub key: T,
    parent: Weak<RefCell<TreeNode<T>>>,
    left: RedBlackTree<T>,
    right: RedBlackTree<T>,
}
#[derive(Debug, Clone)]
pub struct RedBlackTree<T> {
    node: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T: Ord + Debug > RedBlackTree<T> {
    pub fn new() -> RedBlackTree<T> { // create a new node
        Self {node: None}
    }

    pub fn get_node_dir(pnode: Rc<RefCell<TreeNode<T>>>, cnode: Rc<RefCell<TreeNode<T>>> ) -> i32 {
        if pnode.borrow().left.node.is_some() { // left child of parent exists
            if Rc::ptr_eq(pnode.borrow().left.node.as_ref().unwrap(), &cnode) {
                return 0;
            }
        }
        if pnode.borrow().right.node.is_some() {// right child of parent exists
            if Rc::ptr_eq(pnode.borrow().right.node.as_ref().unwrap(), &cnode) {
                return 1;
            }
        }
        return -1;
    } 

    pub fn find_node(&mut self, key: T) -> bool {
        let cur_node = &mut self.node;
        match cur_node {
            &mut None => return false,
            &mut Some(ref mut pnode) => { // node exists
                let pclone = Rc::clone(&pnode);
                if pnode.borrow().key == key { // found node
                    return true;
                }
                let pborrow = pclone.borrow_mut();
                if key < pborrow.key { // go left
                    match pborrow.left.node {
                        Some(ref temp) => {
                            return RedBlackTree{node: Some(Rc::clone(temp))}.find_node(key);
                        },
                        None => return false,
                    }
                } else { // go right
                    match pborrow.right.node {
                        Some(ref temp) => {
                            return RedBlackTree{node: Some(Rc::clone(temp))}.find_node(key);
                        },
                        None => return false,
                    }
                };
            
            }
        }
    }

    pub fn insert_node(&mut self, key: T) {
        fn rec_find_node<T: Ord + Debug >(RBTree: &mut RedBlackTree<T>, key: T) -> Option<Rc<RefCell<TreeNode<T>>>> {
            let cur_node = &mut RBTree.node;
            match cur_node {
                &mut None => { // node does not exist should be the root
                    *cur_node = Some(Rc::new(RefCell::new(TreeNode {
                        color: NodeColor::Black,
                        key,
                        parent: Weak::new(),
                        left: RedBlackTree{node: None},
                        right: RedBlackTree{node: None},
                    })));
                    return None
                },
                &mut Some(ref mut pnode) => { // node exists
                    let pclone = Rc::clone(&pnode);
                    // let mut pborrow = pclone.borrow_mut();
                    let color = NodeColor::Red;
                    if pnode.borrow().key == key { // key already exists in tree
                        return None
                    }
                    let mut new_node = RedBlackTree::new();
                    {
                        let mut pborrow = pclone.borrow_mut();
                        
                        if key < pborrow.key { // go left
                            match pborrow.left.node {
                                Some(ref temp) => {

                                    return rec_find_node(&mut RedBlackTree{node: Some(Rc::clone(temp))}, key);
                                },
                                None => {
                                    new_node.node = Some(Rc::new(RefCell::new(TreeNode {
                                        color: color,
                                        key,
                                        parent: Rc::downgrade(&pnode),
                                        left: RedBlackTree{node: None},
                                        right: RedBlackTree{node: None},
                                    })));
                                    pborrow.left = new_node;
                                    return Some(Rc::clone(pborrow.left.node.as_ref().unwrap()));
                                },
                            }
                        } else { // go right
                            match pborrow.right.node {
                                Some(ref temp) => {
                                    return rec_find_node(&mut RedBlackTree{node: Some(Rc::clone(temp))}, key);
                                },
                                None => {
                                    new_node.node = Some(Rc::new(RefCell::new(TreeNode {
                                        color: color,
                                        key,
                                        parent: Rc::downgrade(&pnode),
                                        left: RedBlackTree{node: None},
                                        right: RedBlackTree{node: None},
                                    })));
                                    pborrow.right = new_node;
                                    return Some(Rc::clone(pborrow.right.node.as_ref().unwrap()));
                                },
                            }
                        };
                    }
                }
            }
        }
        fn loop_insert_node<T: Ord + Debug >(root: &mut RedBlackTree<T>, ctree: Option<Rc<RefCell<TreeNode<T>>>>) -> Option<Rc<RefCell<TreeNode<T>>>> {
            match ctree {
                None => return None,
                Some(ref cnode) => {
                    // get parent node
                    if !cnode.borrow().parent.upgrade().is_none() {
                        let cclone = Rc::clone(&cnode);
                        let pnode;
                        {
                            let cborrow = cclone.borrow_mut();
                            pnode = Rc::clone(cborrow.parent.upgrade().as_ref().unwrap());
                        }
                        let pclone = Rc::clone(&pnode);
                        let dir;
                        {
                            dir = RedBlackTree::get_node_dir(Rc::clone(&pclone), Rc::clone(cnode));
                        }
                        // case 1: parent node is black
                        if pnode.borrow().color == NodeColor::Black {
                            //println!("case 1: parent node is black");
                            return None;
                        } // parent node is red
                        // get grandparent node
                        let gnode;
                        {
                            let pborrow = pclone.borrow_mut();
                            gnode = Weak::clone(&pborrow.parent);
                        }
                        match gnode.upgrade() {
                            None => { // case 4: parent is red and the root
                                //println!("case 4: parent is red and the root");
                                let mut pborrow = pclone.borrow_mut();
                                pborrow.color = NodeColor::Black;
                                return None;
                            }
                            Some(ref mut g) => { // node has grandparent
                                // get uncle
                                let unode; // take uncle away then give back later
                                let udir;
                                {
                                    udir = RedBlackTree::get_node_dir(Rc::clone(g), Rc::clone(&pclone));
                                }
                                //println!("{}", udir);
                                if udir == 0 { // left
                                    //println!("right uncle");
                                    let pborrow;
                                    let temp;
                                    {
                                        pborrow = pclone.borrow_mut();
                                        temp = Rc::clone(pborrow.parent.upgrade().as_ref().unwrap());
                                    }
                                    unode = temp.borrow_mut().right.node.take();
                                } else { // right
                                    //println!("left uncle");
                                    let pborrow;
                                    let temp;
                                    {
                                        pborrow = pclone.borrow_mut();
                                        temp = Rc::clone(pborrow.parent.upgrade().as_ref().unwrap());
                                    }
                                    unode = temp.borrow_mut().left.node.take();
                                }
                                //println!("{}", unode.is_none());
                                if unode.is_none() || unode.as_ref().unwrap().borrow_mut().color == NodeColor::Black { // case 5-6
                                    let mut cond = false;
                                    if udir == 1 {
                                        if pclone.borrow().left.node.is_some() {
                                            cond = Rc::ptr_eq(&cclone, pclone.borrow().left.node.as_ref().unwrap());
                                        } else {
                                            cond = false;
                                        }
                                    } else if udir == 0{
                                        if pclone.borrow().right.node.is_some() {
                                            cond = Rc::ptr_eq(&cclone, pclone.borrow().right.node.as_ref().unwrap());
                                        } else {
                                            cond = false;
                                        }
                                    }
                                    if cond { // case 5: p is red && u is black && c inner grandchild of g
                                        //println!("case 5: p is red && u is black && c inner grandchild of g");
                                        if dir == 0 { // rotate right
                                            let mut ptree = RedBlackTree{node: Some(Rc::clone(&pclone))};
                                            ptree.rotateRight();
                                        } else if dir == 1 { // rotate left
                                            let mut ptree = RedBlackTree{node: Some(Rc::clone(&pclone))};
                                            ptree.rotateLeft();
                                        }
                                        insert_node_case6(root, Rc::clone(g), Rc::clone(&cclone), dir)
                                    } else {
                                        insert_node_case6(root, Rc::clone(g), Rc::clone(&pclone), 1-dir)
                                    }
                                    // case 6: current node is outer grandchild where P is red && uncle is black
                                    fn insert_node_case6<T: Ord + Debug >(root: &mut RedBlackTree<T>, parent: Rc<RefCell<TreeNode<T>>>, child: Rc<RefCell<TreeNode<T>>>, dir: i32) {
                                        //println!("case 6");
                                        if dir == 0 { // rotate left
                                            RedBlackTree{node: Some(Rc::clone(&parent))}.rotateLeft();
                                        } else if dir == 1 { // rotate right
                                            RedBlackTree{node: Some(Rc::clone(&parent))}.rotateRight();
                                        }
                                        child.borrow_mut().color = NodeColor::Black;
                                        parent.borrow_mut().color = NodeColor::Red;
                                        // check to replace root
                                        if Rc::ptr_eq(root.node.as_ref().unwrap(), &parent) {
                                            root.node = Some(child);
                                        }
                                    }

                                    { // give uncle node back to tree
                                        if udir == 0 {
                                            g.borrow_mut().right = RedBlackTree{node: unode};
                                        } else {
                                            g.borrow_mut().left = RedBlackTree{node: unode};
                                        }

                                        return None;
                                    }
                                } else { // case 2: parent and uncle are red
                                    //println!("case 2: parent and uncle are red");
                                    pclone.borrow_mut().color = NodeColor::Black;
                                    unode.as_ref().unwrap().borrow_mut().color = NodeColor::Black;
                                    // check if grandparent is root
                                    if Rc::ptr_eq(root.node.as_ref().unwrap(), g) {

                                    } else {
                                        g.borrow_mut().color = NodeColor::Red;
                                    }
                                    { // give uncle node back to tree
                                        if udir == 0 {
                                            g.borrow_mut().right = RedBlackTree{node: unode};
                                        } else {
                                            g.borrow_mut().left = RedBlackTree{node: unode};
                                        }
                                    }
                                    //rec_insert_node(&mut gnode.upgrade(), dir)
                                    return Some(Rc::clone(&g));
                                }
                            }
                        }
                    } else { // case 3: current node is root of tree and red
                        //println!("case 3: current node is root of tree and red");
                        cnode.borrow_mut().color = NodeColor::Black;
                        return None;
                    }
                },
            }
        }
        let mut copt = rec_find_node(self, key);
        match copt {
            None => return,
            Some(ref mut cnode) => {
                let mut i = 0;
                let mut temp = None;
                let mut cont = true;
                while cont {
                    if i == 0 {
                        temp = loop_insert_node(self, Some(Rc::clone(cnode)));
                        i+=1;
                    } else {
                        i+=1;
                        temp = loop_insert_node(self, Some(Rc::clone(&temp.as_ref().unwrap())));
                    }
                    if temp.is_none() {
                        cont = false;
                    } else {
                    }
                }
            }
        }
    }

    pub fn delete_node(&mut self, key: T) -> bool {
        fn rec_find_smallest<T: Ord + Debug>(node: &mut Rc<RefCell<TreeNode<T>>>) -> Rc<RefCell<TreeNode<T>>> {
            match node.borrow_mut().left.node {
                None => {return Rc::clone(node)}, // will return current node
                Some(ref mut rcleftnode) => {
                    match rcleftnode.borrow_mut().left.node {
                        None => {

                        }, // will return current node
                        Some(ref mut left) => {
                            return rec_find_smallest(left)
                        },
                    }
                },
            }
            return Rc::clone(node.borrow_mut().left.node.take().as_ref().unwrap());
        }
        fn rec_find_node<T: Ord + Debug >(RBTree: &mut RedBlackTree<T>, key: T) -> Option<Rc<RefCell<TreeNode<T>>>> {
            let cur_node = &mut RBTree.node;
            match cur_node {
                &mut None => return None,
                &mut Some(ref mut pnode) => { // node exists
                    let pclone = Rc::clone(&pnode);
                    // let mut pborrow = pclone.borrow_mut();
                    let color = NodeColor::Red;
                    if pnode.borrow().key == key { // found node
                        return Some(Rc::clone(&pnode));
                    }
                
                    let mut pborrow = pclone.borrow_mut();
                    
                    if key < pborrow.key { // go left
                        match pborrow.left.node {
                            Some(ref temp) => {
                                return rec_find_node(&mut RedBlackTree{node: Some(Rc::clone(temp))}, key);
                            },
                            None => return None,
                        }
                    } else { // go right
                        match pborrow.right.node {
                            Some(ref temp) => {
                                return rec_find_node(&mut RedBlackTree{node: Some(Rc::clone(temp))}, key);
                            },
                            None => return None,
                        }
                    };
                
                }
            }
        }
        fn loop_delete_node<T: Ord + Debug >(root: &mut RedBlackTree<T>, node: Option<Rc<RefCell<TreeNode<T>>>>, dir: i32) {
            let rcnode = node.as_ref().unwrap();
            let rcclone = Rc::clone(&rcnode);
            let mut parent;
            {
                let rcborrow = rcclone.borrow_mut();
                parent = Weak::clone(&rcborrow.parent).upgrade();
            }
            if parent.is_none() { // node is root -----------------------------
                let mut rightchild;
                let mut leftchild;
                {
                    let rcborrow = rcclone.borrow_mut();
                    let rightopt = rcborrow.right.node.as_ref();
                    let leftopt = rcborrow.left.node.as_ref();
                    if leftopt.is_none() && rightopt.is_none() { // root has no children
                        root.node = None;
                        return;
                    }  else if leftopt.is_some() && rightopt.is_none() { // only left child
                        leftopt.unwrap().borrow_mut().color = NodeColor::Black;
                        root.node = Some(Rc::clone(leftopt.unwrap()));
                        return;
                    } else if leftopt.is_none() && rightopt.is_some() { // only right child
                        rightopt.unwrap().borrow_mut().color = NodeColor::Black;
                        root.node = Some(Rc::clone(rightopt.unwrap()));
                        return;
                    } else { // root has both children
                        rightchild = Rc::clone(rightopt.unwrap());
                        leftchild = Rc::clone(leftopt.unwrap());
                    }
                }
                // case where both children exist
                let replacement = rec_find_smallest(&mut Rc::clone(&rightchild));
                if Rc::ptr_eq(&replacement, &rightchild) { // right child has no left children
                    leftchild.borrow_mut().parent = Weak::clone(&Rc::downgrade(&replacement));
                    replacement.borrow_mut().left = RedBlackTree{node: Some(leftchild)};
                    replacement.borrow_mut().color = rcnode.borrow_mut().color;
                    return;
                }
                else if replacement.borrow_mut().right.node.is_some() { // if replacement node has right child replace itself with its right child in the tree
                    let reparent;
                    let reright;
                    {reparent = Rc::clone(replacement.borrow_mut().parent.upgrade().as_ref().unwrap());}
                    reright = replacement.borrow_mut().right.node.take();
                    reright.as_ref().unwrap().borrow_mut().color = replacement.borrow_mut().color;
                    reparent.borrow_mut().left = RedBlackTree{node: reright}; // replace node with its right child
                    rightchild.borrow_mut().parent = Weak::clone(&Rc::downgrade(&replacement));
                    leftchild.borrow_mut().parent = Weak::clone(&Rc::downgrade(&replacement));
                }
                replacement.borrow_mut().right = RedBlackTree{node: Some(rightchild)};
                replacement.borrow_mut().left = RedBlackTree{node: Some(leftchild)};
                replacement.borrow_mut().color = rcnode.borrow_mut().color;
                root.node = Some(replacement);
                return;
            }
            // node is not root --------------------------
            let mut pnode = parent.as_ref().unwrap();
            let mut rightchild;
            let mut leftchild;
            {
                let rcborrow = rcclone.borrow_mut();
                let rightopt = rcborrow.right.node.as_ref();
                let leftopt = rcborrow.left.node.as_ref();
                if leftopt.is_none() && rightopt.is_none() { // node has no children --------------------------------
                    if dir == 0 {
                        pnode.borrow_mut().left.node = None;
                    } else if dir == 1 {
                        pnode.borrow_mut().right.node = None;
                    }
                    return;
                } else if leftopt.is_some() && rightopt.is_none() { // only left child ------------------------------
                    leftopt.unwrap().borrow_mut().color = NodeColor::Black;
                    leftopt.unwrap().borrow_mut().parent = Weak::clone(&rcborrow.parent);
                    if dir == 0 {
                        pnode.borrow_mut().left.node = Some(Rc::clone(leftopt.unwrap()));
                    } else if dir == 1 {
                        pnode.borrow_mut().right.node = Some(Rc::clone(leftopt.unwrap()));
                    }
                    return;
                } else if leftopt.is_none() && rightopt.is_some() { // only right child
                    rightopt.unwrap().borrow_mut().color = NodeColor::Black;
                    rightopt.unwrap().borrow_mut().parent = Weak::clone(&rcborrow.parent);
                    if dir == 0 {
                        pnode.borrow_mut().left.node = Some(Rc::clone(rightopt.unwrap()));
                    } else if dir == 1 {
                        pnode.borrow_mut().right.node = Some(Rc::clone(rightopt.unwrap()));
                    }
                    return;
                } else { // root has both children
                    rightchild = Rc::clone(rightopt.unwrap());
                    leftchild = Rc::clone(leftopt.unwrap());
                }
            }
            let replacement = rec_find_smallest(&mut Rc::clone(&rightchild));
            if Rc::ptr_eq(&replacement, &rightchild) { // right child has no left children
                leftchild.borrow_mut().parent = Weak::clone(&Rc::downgrade(&replacement));
                replacement.borrow_mut().left = RedBlackTree{node: Some(leftchild)};
                replacement.borrow_mut().color = rcnode.borrow_mut().color;
                if dir == 0 {
                    pnode.borrow_mut().left.node = Some(replacement);
                } else if dir == 1 {
                    pnode.borrow_mut().right.node = Some(replacement);
                }
                return;
            }
            else if replacement.borrow_mut().right.node.is_some() { // if replacement node has right child replace itself with its right child in the tree
                let reparent;
                let reright;
                {reparent = Rc::clone(replacement.borrow_mut().parent.upgrade().as_ref().unwrap());}
                reright = replacement.borrow_mut().right.node.take();
                reright.as_ref().unwrap().borrow_mut().color = replacement.borrow_mut().color;
                reparent.borrow_mut().left = RedBlackTree{node: reright}; // replace node with its right child
                rightchild.borrow_mut().parent = Weak::clone(&Rc::downgrade(&replacement));
                leftchild.borrow_mut().parent = Weak::clone(&Rc::downgrade(&replacement));
            }
            replacement.borrow_mut().right = RedBlackTree{node: Some(rightchild)};
            replacement.borrow_mut().left = RedBlackTree{node: Some(leftchild)};
            replacement.borrow_mut().color = rcnode.borrow_mut().color;
            if dir == 0 {
                pnode.borrow_mut().left.node = Some(replacement);
            } else if dir == 1 {
                pnode.borrow_mut().right.node = Some(replacement);
            }
            return;
        }
        // main fn
        let mut opt = rec_find_node(self, key);
        match opt {
            None => return false,
            Some(ref mut node) => {
                let parent;
                let mut dir = -1;
                {
                    parent = node.borrow_mut().parent.upgrade().take();
                }
                if parent.is_some() {
                    dir = RedBlackTree::get_node_dir(Rc::clone(parent.as_ref().unwrap()), Rc::clone(&node));
                    if dir == 0 {
                        parent.as_ref().unwrap().borrow_mut().left.node = None;
                    } else if dir == 1 {
                        parent.as_ref().unwrap().borrow_mut().right.node = None;
                    }
                }
                loop_delete_node(self, Some(Rc::clone(node)), dir);
                return true;
            }
        }

        return false
    }

    /* Summary
    child = root.right
    if root.parent exists {
        getdir(root.parent, parent)
        if left {
            root.parent.left = child
        } else {
            root.parent.right = child
        }
    }
    root.right = child.left
    child.left = root
    child.parent = root.parent
    root.parent = child
    return child
    */
    pub fn rotateLeft(&mut self) { // -> RedBlackTree<T> { // return root
        let root = &mut self.node;
        match root {
            None => {},
            Some(ref mut rcroot) => {
                let child = Rc::clone(rcroot.borrow_mut().right.node.take().as_ref().unwrap()); // child = root.right
                if rcroot.borrow_mut().parent.upgrade().is_some() {
                    let dir = RedBlackTree::get_node_dir(Rc::clone(rcroot.borrow_mut().parent.upgrade().as_ref().unwrap()), Rc::clone(rcroot));
                    if dir == 0 {
                        rcroot.borrow_mut().parent.upgrade().as_ref().unwrap().borrow_mut().left = RedBlackTree{node: Some(Rc::clone(&child))};
                    } else if dir == 1 {
                        rcroot.borrow_mut().parent.upgrade().as_ref().unwrap().borrow_mut().right = RedBlackTree{node: Some(Rc::clone(&child))};
                    }
                }
                rcroot.borrow_mut().right = RedBlackTree{node: child.borrow_mut().left.node.take()}; // root.right = child.left
                child.borrow_mut().left = RedBlackTree{node: Some(Rc::clone(&rcroot))}; // child.left = root
                child.borrow_mut().parent = Weak::clone(&rcroot.borrow_mut().parent); // child.parent = root.parent
                rcroot.borrow_mut().parent = Rc::downgrade(&child); // root.parent = child
                self.node = Some(child);
            },
        }
    }

    /* Summary
    child = root.left
    if root.parent exists {
        getdir(root.parent, parent)
        if left {
            root.parent.left = child
        } else {
            root.parent.right = child
        }
    }
    root.left = child.right
    child.right = root
    return child
    */
    pub fn rotateRight(&mut self) { // -> RedBlackTree<T> { // return root
        let root = &mut self.node;
        match root {
            None => {},
            Some(ref mut rcroot) => {
                let child = Rc::clone(rcroot.borrow_mut().left.node.take().as_ref().unwrap()); // child = root.left
                if rcroot.borrow_mut().parent.upgrade().is_some() {
                    let dir = RedBlackTree::get_node_dir(Rc::clone(rcroot.borrow_mut().parent.upgrade().as_ref().unwrap()), Rc::clone(rcroot));
                    if dir == 0 {
                        rcroot.borrow_mut().parent.upgrade().as_ref().unwrap().borrow_mut().left = RedBlackTree{node: Some(Rc::clone(&child))};
                    } else if dir == 1 {
                        rcroot.borrow_mut().parent.upgrade().as_ref().unwrap().borrow_mut().right = RedBlackTree{node: Some(Rc::clone(&child))};
                    }
                }
                rcroot.borrow_mut().left = RedBlackTree{node: child.borrow_mut().right.node.take()}; // root.left = child.right
                child.borrow_mut().right = RedBlackTree{node: Some(Rc::clone(&rcroot))}; // child.right = root
                child.borrow_mut().parent = Weak::clone(&rcroot.borrow_mut().parent); // child.parent = root.parent
                rcroot.borrow_mut().parent = Rc::downgrade(&child); // root.parent = child
                self.node = Some(child);
            },
        }
    }

    pub fn printInOrder(&mut self) {

        let cur_node = &mut self.node;
    
        match cur_node {
            &mut None => { // node does not exist should be the root
                return;
            },
            &mut Some(ref node) => { // node exists
                let mut treenode = node.borrow_mut();
                
                // first recur on left child
                (treenode.left).printInOrder();
    
                // then print the data of node
                print!("{:?}, ", treenode.key);
                // now recur on right child
                (treenode.right).printInOrder();
    
            }
        }
    }

    pub fn getLeavesCount(&mut self, mut counter: u32) -> u32 {

        let mut cur_node = &mut self.node;
    
        match cur_node {
            &mut None => { // node does not exist should be the root
                counter= counter+1;
                return counter;
            },
            &mut Some(ref mut node) => { // node exists
                let mut treenode = node.borrow_mut();
            
                counter = (treenode.left).getLeavesCount(counter);
                counter = (treenode.right).getLeavesCount(counter);

                return counter;
    
            }
        }
    }


    pub fn getTreeHeight(&mut self) -> i32 {

        let mut cur_node = &mut self.node;
    
        match cur_node {
            &mut None => { // node does not exist should be the root
                return -1;
            },
            &mut Some(ref mut node) => { // node exists
                let mut treenode = node.borrow_mut();
            
                return max((treenode.left).getTreeHeight(), (treenode.right).getTreeHeight()) + 1;
    
            }
        }
    }

    pub fn isTreeEmpty(&mut self) -> bool {
        return self.node.is_none(); // checks if option block on node is empty
    }

    pub fn printTree(&mut self, space: &mut i32) {

        let mut cur_node = &mut self.node;
    
        match cur_node {
            &mut None => { // node does not exist should be the root
                return;
            },
            &mut Some(ref mut node) => { // node exists

                let mut treenode = node.borrow_mut();
            
                *space+=5;
                (treenode.right).printTree(space);
    
                println!("");
    
                for i in 5..*space {
                    print!(" ");
                }
                match treenode.color {
                    NodeColor::Black => println!("B:{:?}", treenode.key),
                    NodeColor::Red => println!("R:{:?}", treenode.key),
                }
                //println!("{:?}", treenode.key);
    
                (treenode.left).printTree(space);
    
                *space-=5;

            }
        }
    }
}


fn main() {
    let mut tree = RedBlackTree::<u32>::new(); // create an empty tree

    println!("Red Black Tree Interface :");

        println!("--------------------------------------------------------");
        println!("Command Directory :");
        println!("Insert a node to the red-black tree : 1 <node>");
        println!("Delete a node from the red-black tree : 2 <node>");
        println!("Count the number of leaves in a tree : 3");
        println!("Return the height of a tree : 4");
        println!("Print In-order traversal of the tree : 5");
        println!("Check if the tree is empty : 6");
        println!("Print the tree : 7");
        println!("Exit : 0");
        println!("--------------------------------------------------------");

    loop{

        println!("Enter Command :");
        let mut userInput = String::new();
    
        io::stdin().read_line(&mut userInput).expect("failed to readline");

        if let Some('\n')=userInput.chars().next_back() {
            userInput.pop();
        }
        if let Some('\r')=userInput.chars().next_back() {
            userInput.pop();
        }
       
        //https://stackoverflow.com/questions/34090639/how-do-i-convert-a-vector-of-strings-to-a-vector-of-integers-in-a-functional-way
        let userInputVecInt: Vec<i32> = userInput.split(" ").map(|x| x.parse::<i32>().unwrap()).collect();
        let userInputVecData: Vec<u32> = userInput.split(" ").map(|x| x.parse::<u32>().unwrap()).collect();

        let _=stdout().flush();

        if userInputVecInt.len()==1{
            if userInputVecInt[0] == 3{
                let mut counter = 0; 
                counter = tree.getLeavesCount(counter);
                println!("Leaf Count is {}",counter/2);
            }else if userInputVecInt[0] == 4{
                let height = tree.getTreeHeight();
                println!("Height of tree {}",height);

            }else if userInputVecInt[0] == 5{
                println!("Print In-order traversal of tree");
                tree.printInOrder();
                
            }else if userInputVecInt[0] == 6{
                println!("Tree is empty - {}",tree.isTreeEmpty());
                
            }else if userInputVecInt[0] == 7{
                tree.printTree(&mut 0);
            }else if userInputVecInt[0] == 0{
                process::exit(1);
            }

        } else if userInputVecInt.len() ==2{
            if userInputVecData[0] == 1{
                tree.insert_node(userInputVecData[1]);
            }else if userInputVecData[0] == 2{
                tree.delete_node(userInputVecData[1]);
            }
        }else{
            print!("Please enter commad in correct format");
        }
    }
}
