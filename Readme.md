# ECE 421 - Project 2

## Running Instructions

To run the AVL tree command-line interface:
```sh
$ cd AVLTree/
$ cargo run
```

To run the Red-black tree command-line interface:
```sh
$ cd RBTree/
$ cargo run
```

The commad-line interface will provide command instructions as following to the user, 

 Command Directory :
        Insert a node to the red-black tree : 1 <node>
 
        Delete a node from the red-black tree : 2 <node>
 
        Count the number of leaves in a tree : 3
 
        Return the height of a tree : 4
 
        Print In-order traversal of the tree : 5
 
        Check if the tree is empty : 6
 
        Print the tree : 7
 
        Exit : 0
 

 ## Detailed Instructions after implementing running instructions: 
  
 1) To Add a node: 1 <node>
        eg: to add 8 -> 1 8
 2) To delete a node: 2 <node>
        eg: to delete 8 -> 2 8
 3) To cound the number of leaves -> 3
 4) To get the height of a tree -> 4
 5) to print tree with in-order treversal -> 5
 6) To check if tree is empty of now -> 6
 7) To print the tree -> 7
 8) To exit the command-line interface -> 0
 
 Both AVL and RBTree follow same command line instructions. 
  
 
 ## Instructions for Benchmarking 
  
```sh
$ cd RBTree/
$ cargo bench
```
```sh
$ cd AVLTree/
$ cargo bench
```
  
Reports can be found at following directory:
 
  avl_tree/target/creterion/report/index.html
 
  red_black_tree/target/creterion/report/index.html

Note: index.html need a web browser to run. 
 
  
