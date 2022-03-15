use std::cell::RefCell;
use std::rc::Rc;
use core::cmp::max;
use std::fmt::{self, Debug};
use std::marker::Copy;
use std::num;
use std::io::stdin;
use std::io;
use std::io::stdout;
use std::io::Write;
use std::process;


#[derive(Debug, Clone)]
pub struct TreeNode<T>  {
    pub key: T,
    pub balance_factor: i32,
    left: AVLTree<T>,
    right: AVLTree<T>,
}
#[derive(Debug, Clone)]
pub struct AVLTree<T> {
    node: Option<Rc<RefCell<TreeNode<T>>>>,
}

// unpack => 

impl<T: Ord + Debug + std::marker::Copy> AVLTree<T> {
    pub fn new() -> AVLTree<T> { // create a new node
        Self {node: None}
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
                            return AVLTree{node: Some(Rc::clone(temp))}.find_node(key);
                        },
                        None => return false,
                    }
                } else { // go right
                    match pborrow.right.node {
                        Some(ref temp) => {
                            return AVLTree{node: Some(Rc::clone(temp))}.find_node(key);
                        },
                        None => return false,
                    }
                };
            
            }
        }
    }

    pub fn insert_node(&mut self, key: T) {
        fn rec_insert_node<T: Ord + Debug + Copy>(avl_tree: &mut AVLTree<T>, key: T) -> bool{
            let mut return_val:bool = false;
            let mut prewv_return_val:bool = false;
            let cur_node;
            {
                cur_node = &mut avl_tree.node;
            }
            match cur_node {
                &mut None => { // node does not exist should be the root
                    *cur_node = Some(Rc::new(RefCell::new(TreeNode {
                        key,
                        balance_factor: 0,
                        left: AVLTree{node: None},
                        right: AVLTree{node: None},
                    })));
                    return_val = true;
                },
                &mut Some(ref mut node) => { // node exists
                    let mut treenode = node.borrow_mut();
                    let bf:i32 = treenode.balance_factor;
                    if treenode.key == key { // key already exists in tree
                        return_val =  false;
                    }// get node from child tree
                    else if key < treenode.key { // left
                        
                        let mut new_node = &mut treenode.left;
                        match new_node.node {
                            Some(_) => {
                                let temp:bool;
                                
                                {
                                    prewv_return_val = rec_insert_node(new_node, key);
                                }
                                if(prewv_return_val){
                                    
                                    {
                                        if(bf == 0){
                                            return_val = true;
                                        }
                                        else{
                                            return_val = false;
                                        }
                                    }
                                    
                                    {
                                        treenode.balance_factor-=1;
                                    }
                                }                                
                                
                            },
                            None => {
                                // make new node
                                new_node.node = Some(Rc::new(RefCell::new(TreeNode {
                                    key,
                                    balance_factor: 0,
                                    left: AVLTree{node: None},
                                    right: AVLTree{node: None},
                                })));
                                if(treenode.balance_factor == 0){
                                    return_val =  true;
                                }
                                else{
                                    return_val = false;
                                }
                                {
                                    treenode.balance_factor-=1;
                                }
                                
                            },
                        }
                    } else { // right
                        let mut new_node = &mut treenode.right;
                        match new_node.node {
                            Some(_) => {
                                let temp:bool;
                                {
                                    prewv_return_val = rec_insert_node(new_node, key);
                                }
                                if(prewv_return_val){
                                    
                                    if(bf == 0){
                                        return_val = true;
                                    }
                                    else{
                                        return_val = false;
                                    }
                                   
                                    {
                                        treenode.balance_factor+=1;
                                    }
                                    
                                    
                                }
                                
                            },
                            None => {
                                // make new node
                                new_node.node = Some(Rc::new(RefCell::new(TreeNode {
                                    key,
                                    balance_factor: 0,
                                    left: AVLTree{node: None},
                                    right: AVLTree{node: None},
                                })));
                                if(treenode.balance_factor == 0){
                                    return_val =  true;
                                }
                                else{
                                    return_val = false;
                                }
                                {
                                    treenode.balance_factor+=1;
                                }
                                
                            },
                        }
                    }
                },
                
            }
            if(prewv_return_val|| return_val){
                {
                    avl_tree.balanceTree();
                }
                return return_val;
            }
            else{
                return false;
            }

        }
        if(rec_insert_node(self, key)){
            self.balanceTree();
        } // only root node exists beyond this point
    }

    pub fn delete_node(&mut self, key: T) {
        fn rec_delete_node<T: Ord + Debug + Copy>(avl_tree: &mut AVLTree<T>, key: T) -> bool{
            let mut return_val:bool = false;
            let mut prewv_return_val:bool = false;
            let cur_node;
            let mut bfchange:i32 = 0;
            {
                cur_node = &mut avl_tree.node;
            }
            match cur_node {
                &mut None => {
                    //if the tree is empty, do nothing
                }
                &mut Some(_) => {
                    //tree is not empty
                    let mut node;
                    {
                        node = Rc::clone(&cur_node.as_ref().unwrap());
                    }
                    let mut val;
                    {
                        val = node.borrow_mut().key;
                    }
                    let mut bf;
                    {
                        bf = node.borrow_mut().balance_factor;
                    }
                    if(key<val){
                        //traverse left subtree if exists
                        let mut new_node = &mut node.borrow_mut().left;
                        match new_node.node{
                            None => {
                                // cant traverse any further, node doesnt exist
                            }
                            Some(_) => {
                                // traverse left subtree
                                prewv_return_val = rec_delete_node(new_node,key);
                                if(prewv_return_val){
                                    {
                                        bfchange=1;
                                    }
                                    {
                                        bf = bf + bfchange
                                    }
                                    if(bf == 0){
                                        return_val = true;
                                    }
                                    else{
                                        return_val = false;
                                    }
                                }
                            }
                        }

                    }
                    else if (key>val){
                        //traverse right subtree if exists
                        let mut new_node = &mut node.borrow_mut().right;
                        match new_node.node{
                            None => {
                                // cant traverse any further, node doesnt exist
                            }
                            Some(_) => {
                                // traverse right subtree
                                prewv_return_val = rec_delete_node(new_node,key);
                                if(prewv_return_val){
                                    {
                                        bfchange=-1
                                    }
                                    {
                                        bf = bf + bfchange;
                                    }
                                    if(bf == 0){
                                        return_val = true;
                                    }
                                    else{
                                        return_val = false;
                                    }
                                }
                            }
                        }

                    }
                    else{
                        //found node to delete
                        let mut left_is_empty:bool = false;
                        let mut right_is_empty:bool = false;
                        {
                            let mut left = &mut node.borrow_mut().left;
                            match left.node{
                                None => {
                                    left_is_empty = true;
                                }
                                Some(_)=> {
                                }
                            }
                        }
                        {
                            let mut right = &mut node.borrow_mut().right;
                            match right.node{
                                None => {
                                    right_is_empty = true;
                                }
                                Some(_)=> {
                                }
                            }
                        }
                        if(left_is_empty&&right_is_empty){
                            //case 1 node has no children, just set node to none
                            *avl_tree = AVLTree::<T>::new();
                            return_val = true;
                        }
                        //case 2 and 3, node has 1 child, replace node with child
                        else if (!left_is_empty&&right_is_empty){
                            *avl_tree = AVLTree{node:Some(Rc::clone(node.borrow_mut().left.node.as_ref().unwrap()))};
                            return_val = true;
                        }
                        else if (left_is_empty&&!right_is_empty){
                            *avl_tree = AVLTree{node:Some(Rc::clone(node.borrow_mut().right.node.as_ref().unwrap()))};
                            return_val = true;
                        }
                        //case 4, node has 2 children, replace with next inorder member
                        else{
                            let mut val = AVLTree{node:Some(Rc::clone(node.borrow_mut().right.node.as_ref().unwrap()))}.find_next_inorder_value();
                            {
                                node.borrow_mut().key = val;
                            }
                            let mut new_node = &mut node.borrow_mut().right;
                            match new_node.node{
                                None => {
                                    // cant traverse any further, node doesnt exist
                                }
                                Some(_) => {
                                    // traverse right subtree
                                    prewv_return_val = rec_delete_node(new_node,val);
                                    if(prewv_return_val){
                                        {
                                            bfchange=-1
                                        }
                                        {
                                            bf = bf + bfchange;
                                        }
                                        if(bf == 0){
                                            return_val = true;
                                        }
                                        else{
                                            return_val = false;
                                        }
                                    }
                                }
                            }
                        }
                        
                    }
                {
                    node.borrow_mut().balance_factor+=bfchange;
                }

                }
            }
            if(prewv_return_val){
                {
                    avl_tree.balanceTree();
                }
                return return_val;
            }
            else{
                return return_val;
            }
        }
        {
            rec_delete_node(self, key);
        }
        self.balanceTree();
    }

    fn find_next_inorder_value(&mut self) -> T{
        // function wont be called on a null node
        {
        let mut left = &mut self.node.as_ref().unwrap().borrow_mut().left;
        match left.node{
            None => {
                
            }
            Some(_)=> {
                return left.find_next_inorder_value();
            }
        }
        }
        return self.node.as_ref().unwrap().borrow_mut().key;

    }
    pub fn balanceTree(&mut self) {
        match self.node {
            None => { // node does not exist should be the root
                return;
            },
            Some(ref mut temp) => { // node exists
                let mut node;
                {
                    node = Rc::clone(temp);
                }
                let bf:i32;
                {
                    bf = node.borrow_mut().balance_factor;
                }
                if bf > 1 { // too many in right (right heavy)
                    let mut a:i32 = 0;
                    let mut b:i32 = 0;
                    let mut change_type:i8 = -1;
                    {
                        let mut righttree = &mut node.borrow_mut().right;
                        match righttree.node {
                            None => {},
                            Some(_) => {
                                let mut inode;
                                {
                                    inode = Rc::clone(&righttree.node.as_ref().unwrap())
                                }
                                let ibf:i32;
                                {
                                    ibf = inode.borrow_mut().balance_factor;
                                }
                                if ibf == -1 {
                                    match inode.borrow_mut().left.node.as_ref().unwrap().borrow_mut().balance_factor {
                                        1 => {a = -1; b = 0;},
                                        0 => {a = 0;b = 0;},
                                        -1 =>{ a = 0; b = 1;},
                                        _ => unreachable!(),
                                    };
                                    change_type = 1;
                                    {
                                        inode.borrow_mut().balance_factor = b;
                                    }
                                    {
                                        inode.borrow_mut().left.node.as_ref().unwrap().borrow_mut().balance_factor = 0;
                                    }
                            
                                    righttree.rotateRight();
                                }
                                else{
                                    change_type = 0;
                                    if ibf == 1 { a=0; b=0; } else { a=1; b=-1 };
                                    {
                                        inode.borrow_mut().balance_factor = 0;
                                    }
                                }
                            }
                        }
                    }
                    if(change_type == 1){
                        {
                            node.borrow_mut().balance_factor = a;
                        }
                    }
                    else if (change_type == 0){
                        {
                            node.borrow_mut().balance_factor = a;
                        }
                    }
                    self.rotateLeft();
                } else if bf < -1 { // too many in left (left heavy)
                    let mut a:i32 = 0;
                    let mut b:i32 = 0;
                    let mut change_type:i8 = -1;
                    {   
                        let mut lefttree = &mut node.borrow_mut().left;
                        match lefttree.node {
                            None => {},
                            Some(_) => {
                                let mut inode;
                                {
                                    inode = Rc::clone(&lefttree.node.as_ref().unwrap())
                                }
                                let ibf:i32;
                                
                                {
                                    ibf = inode.borrow_mut().balance_factor;
                                }
                                if ibf == 1 {
                                    change_type = 1;
                                    match inode.borrow_mut().right.node{
                                        None => {a= 0;
                                            b = 0;
                                        },
                                        Some(ref mut iinode) =>{
                                            match iinode.borrow_mut().balance_factor {
                                                -1 => {a=1;b=0;},
                                                0 => {a=0; b=0;},
                                                1 => {a = 0;b = -1},
                                                _ => unreachable!(),
                                            };
                                            iinode.borrow_mut().balance_factor = 0;
                                        }
                                    }
                                    
                                    {
                                        inode.borrow_mut().balance_factor = b;
                                    }
                                    lefttree.rotateLeft();
                                    
                                    
                                    
                                }
                                else{
                                    if ibf == -1 { a = 0; b = 0; } else { a = -1; b = 1 };
                                    change_type = 0;
                                    {
                                        inode.borrow_mut().balance_factor = b;
                                    }
                                    
                                }
                            }
                        }
                        
                    }
                    if(change_type == 1){
                        {
                            node.borrow_mut().balance_factor = a;
                        }
                    }
                    else if (change_type == 0){
                        {
                            node.borrow_mut().balance_factor = a;
                        }
                    }
                    self.rotateRight();
                    
                }
            }
        }
    }

    /* Summary
    child = root.right
    root.right = child.left
    child.left = root
    return child
    */
    pub fn rotateLeft(&mut self) { // -> AVLTree<T> { // return root
        let root = &mut self.node;
        match root {
            None => {},
            Some(ref mut rcroot) => {
                let child = Rc::clone(rcroot.borrow_mut().right.node.take().as_ref().unwrap()); // child = root.right
                rcroot.borrow_mut().right = AVLTree{node: child.borrow_mut().left.node.take()}; // root.right = child.left
                child.borrow_mut().left = AVLTree{node: Some(Rc::clone(&rcroot))}; // child.left = root
                self.node = Some(child);
            },
        }
    }

    /* Summary
    child = root.left
    root.left = child.right
    child.right = root
    return child
    */
    pub fn rotateRight(&mut self) { // -> AVLTree<T> { // return root
        let root = &mut self.node;
        match root {
            None => {},
            Some(ref mut rcroot) => {
                let child = Rc::clone(rcroot.borrow_mut().left.node.take().as_ref().unwrap()); // child = root.left
                rcroot.borrow_mut().left = AVLTree{node: child.borrow_mut().right.node.take()}; // root.left = child.right
                child.borrow_mut().right = AVLTree{node: Some(Rc::clone(&rcroot))}; // child.right = root
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
                //return height of longest child + 1
                return max((treenode.left).getTreeHeight(), (treenode.right).getTreeHeight()) + 1;
    
            }
        }
    }

    pub fn isTreeEmpty(&mut self) -> bool {

        let mut cur_node = &mut self.node;
    
        match cur_node {
            &mut None => { // node does not exist should be the root
                return true;
            },
            &mut Some(ref mut node) => { // node exists
            
                return false;
    
            }
        }
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
                println!("{:?}", treenode.key);
    
                (treenode.left).printTree(space);
    
                *space-=5;

            }
        }
    }
}



fn main() {
    // CLI here


    let mut tree = AVLTree::<u32>::new(); // create an empty tree

    println!("AVL Tree Interface :");
    println!("--------------------------------------------------------");
    println!("Command Directory :");
    println!("Insert a node to the AVL tree : 1 <node>");
    println!("Delete a node from the AVL tree : 2 <node>");
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
