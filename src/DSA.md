# Basic terminology 
 Operator is symbol which is use to mainpulate the data in the code.
 Operand is the data which is easily manipulated with the help of opearator.
- 1 stand for the true .
 - 0 stand for the false.
 - one nit is 4 bytes.
  Function signature consist od the function name parameete and reuturn type and potenitally the visibility.
- fn function_name(param1: Type1, param2: Type2) -> ReturnType {
    // function body}
 - In programming when you use the term `inclusive` it simply means you are considiring the start and end point of the range .
 - Statically generated means that code or data  which is generated at the compile time rather then runtime.
- `Problem` It is a question to which we seek an answer. 
- Algorithm is the step by procedure which can solve any instances of a problem. 
- Sorting means the process of arranging of elements in specific order.Typically based on some predefined criteria.
 # Size
 The size of u8 unsined integer is 1 bytes.
 # Bit-wise operator 
 The opeartor that works at the bit level is bit wise opearator.
  `&(Bitwise AND )`
  It  take two bit if both of them are 1 then it give the output as 1. Else the output is zero.
  ```Rust
  fn main(){
    let x=12;
    let y=26;
    let z=x&y;
    print!("The value of z is {}",z);
    

}
  ```
  ```OR   [|] ```
  In this is there is single 1 also it will take it as one return the value after sum up.


  ```Rust
 fn main(){
    let x=78;
    let y=x;
    let z=x|y;
    print!("The value of z is {}",x);
    print!("The value of z is {}",z);

}
```
`Bitwise XOR : ^`
In this if both the output is zero then it will result 1 as output.
```Rust
fn main(){
 let x: i32=3;
 let y:i32=6;

 print!("The value of y is {}",x^y);
 

}
```
`Left Shift`
In this it will shift two number to the left .
This is wquivalent to the x*2^y and it is applicable to the positive number only .
`Right Shift `
In this it will shift two number to the right .
This is equivalent to the [x/2^y]
```Rust
fn main(){
 let x: i32=4;
 let y:i32=4;

 println!("The value of x is {}",x<<2);
 println!("The value of y is {}",y>>2)
 

}
```
# NOT [`] In rust we use ! for bitwise not operator
It is unary opeartor and it works on the  unary level and it  takes single operand .
```RUST
fn main(){
 let x: i32=5;
 let y:i32=!x;

 println!("The value of x is {}",(x));
 println!("The value of y is {}",y)
 

}
```

# Array 
There are two types of array one is fixed size and other one is dynamically typed . Array of any size can implement the following traits if the element type allows it.
Traits is used to define shared behaviour accros difrent types.
Here we can move elements out of and array with the slice pattern.

- DYNAMICALLY TYPED 
Here size is not known at the compile time.
In this size grow automatically whenever there is need of it .
In this the insertion of data is at the last .
In rust it exist behind the pointer.
Pointer of `Dynamicaaly size type` become wide pointer because it contain the information and pointer both.
- `HEAP Array` 
It is one the example of dynamically typed array.This crates gives the better control on how we want to allocate the space to memory .
The basic difference between pointer and wide pointer is that pointer can address a certain range of memory but wide pointer can address a larger range of memeory address.
   
- A list is the simply a collectiond of items arranged in a particular sequence .

 - In Rust, the while keyword is used to create a loop that continues executing its block of code as long as a specified condition evaluates to true. 
 ```Rust
 fn main(){
    let mut  k1 =5;
    while k1 <14 {
        println!("THE value is good {}",k1);
        k1+= 1;  // It is the shorthand notation for the implementing the variable in k1+1.

    }
}
 ```
 # Exchange sort 
 Exchange sort" is a term that typically refers to sorting algorithms where elements are compared and swapped to arrange them in a specific order, such as ascending or descending. The most well-known exchange sort algorithm is the "Bubble Sort
 # Sequential search Versus Binary Search
    Binary search is efficiant technique of searching and it is quite complex ,as it works on the concept of divide and conquer.
    Time comlexity is O(log n).
    Sequential search is simple technique of searching and it is easy to implement ,it is iterative in nature. Time complexity is O(n).

#  Analysis of Algorithm 
To determine the how efficincy an algorithm work.

# Merge sort 
Algorithm implementation is left 
A process releted to the sorting is merging. By two-way merging we mean a combinig two sorted arrays into one sorted array.
Merge sort involves the following process :-
 - Divide the array into two subarrays each with n/2 items .
 - Conquer(solve) each subarray by sorting it .unless the array is sufficiently small,use recursion to do this .
 - Combine the solutin to the subarrays by merging them into a single sorted arrays.
 
 # Pivot
 In the context of sorting algorithms, a "pivot" is a chosen element from a list that is used to partition the list into two parts during the sorting process. The pivot element serves as a reference point for reorganizing the elements around it.

 # Quick Sort 
  It works on the divide and conquer factor .
  It was developed by Hoare (1962). It is similiar to Mergesort .
  The main fundamental of quicksort is that the array is partitioned by placing all items smaller then the pivot items before that items and all items larger then the pivot items after that.For simplicity we can make any itmem as pivot itmes else we sometime chosse first itme as pivot items.
  Example :- 15,22,13,27,12,10,20,25
             Here we have selected 15 as pivot items and now we have arranged the array in smaller one 
             10,13,12 [All smaller array ] `15` 22,27,20,25 [All larger array]. Now we can sort this accoring to our need.


# Starssen's Matrix Multiplication Algorithm 
 Multiplication of two matrices requires O(N^3) running time but we can reduce this time to O(N^2.81) by using an efficient approach which is known as Strassen Matrix multiplication.
 It works on the concept of divide and conquer.
 It is faster for the loonger matrix. 
 It reduces the number of multiplication.
 Lower time complexity then the traditonally matrix.
  Pseudocode 
  Algorithm MULTIPLY_MATRIX(A, B, C)
for i <- 1 to n do
  for j <- 1 to n do
    C[i][j] <- 0
    for k <- 1 to n do
      C[i][j] <- C[i][j] + A[i][k]*B[k][j]
    end
  end
end
# Larger Integer Multiplication 
Done
# Dynamic programming 
  Dynamic programming is similar to divide-and-conquer in
that an instance of a problem is divided into smaller instances. However, in this
approach we solve small instances first, store the results, and later, whenever we
need a result, look it up instead of recomputing it. The term “dynamic
programming” comes from control theory, and in this sense “programming”
means the use of an array (table) in which a solution is constructed



# Proper implementation begin here 
  # Heaps and stacks
  Rust's types even allow for zero overhead structures,so no additional metadata is stored.
```Rust
use std::mem; // This is the basic module which is use to deal with the he size and alignment of types, initializing and manipulating memory.
struct  Mystruct {  // The size of mystruct is always going to be 4 bytes.
    a :u8,
     b:u8,
    c:u8,
    d:u8,
}
fn main (){
    let mystruct1 = Mystruct {
        a:5,
        b:7,
       c:90,
       d:77,
    
    } ;
    println!("The size of the struct are :{}",mem::size_of::<Mystruct> ());
    println!("THe size of the struct are :{}",mem::size_of::<[Mystruct;2]>());
    println!("My value of the struct are a={},b={},c={},d={}",mystruct1.a,mystruct1.b,mystruct1.c,mystruct1.d);
    
    assert_eq!(mem::size_of::<Mystruct>(),4*mem::size_of::<u8>()); // This calculate the size of single line. As each u8 is of 1 bit . and as soon as the test case is passed the reuslt are shown to you.
    assert_eq!(mem::size_of::<[Mystruct; 2]>(),4*mem::size_of::<u8>()*2); // This calculate the size n time and it will return the test case value after it is passed .if we decress or increase the value it will throw the error.

}
```
The main question is here that why we we are using the predictable size the answer is that we will be storing it stack,and is easy for data locality,instead of pointer dereferencing,the data is stored right at the point of exectuion,making it easy to cache and fast to acces.

Types that don't have predicatble size required heap allocation.Heap allocation come at the considerable cost,as minimizing those typically yeilds great performance cost .
-  Sized and unsized
   - - The best thing is that sized it doestn change with the respect to the data it contains (sized type).  
   - - This isn'case with the unsizned or dynamically sized ,its not easy to store the data on top when working with the stack.
   -  clause means the specific parts or segments of a larger expression or statement.
   The knowlegde of sized vs unsized is especially is usefull when the type is previously unknown- when working with the Rust generics.
# Sized and unsized 
Compiler must know the each type size. The size is imporatant then only we can put other types on the top of the stacks.
`Sized Types` It simple means that size doesn't change with the respect to data it contains. u8 is expample which means that it will always use 32 bit regardless you use 0 or 1000000.
`Unsized Types` It simple means thad size change with the respact to data it contains it may grow or it may shrink.


- `Rust Generics`
Rust provide the implementation of traits with the genrics.

# Copy and cloning
It is simmiliar to the "Send" trait that allows us to sent the type into the multiple threads.
Copy and clonning is the less complex as it is local moving,and it commonly occur when we pass the varibles into a function.
Cloning is  the always deep copy of type-implemented either manually with the clone trait or by using the derive macro.
```Rust
use std::rc::Rc;
fn main () {

    // This is copying the data from one place to the another .It is a implicit.This may become issue when the variable is pointer as it's complitation will fail for sure.
 let y =5;
    let c=y;
    println!("The value is {}",c);
    println!("The value is {}",y);

// This is where clone comes to play .It require the explict implementation of the clone() function to provide the appropriate copy of the type.
    let c=Rc::new(45); // Here we have used the rc std lib for the counting pointer.

    let b=c.clone();
    println!("The value is {}",b);

}
```
## Immutable storage 
It means it state can't be changed after it has been created.
An object created and given a value is assigned some space in memory .The varibles name bound to the object points to that plcae in memory and as long as the memoery is not changed.Mutable happens at the cost of runtime with the frequent cleanup,
## States and resoning 
The states of an object is essentially the current values that its field have at any given time.

# Concurrency and performance 
Imutable codes are more important for the multithreaded system . Immutability is the simple way to avoid all of these runtine and extras costs .
Mutex are the locks of the rust .
Locks and mutux are  bad for the following reasons like panics in mutex zone,and ordering of the data structure.
There are many data structue crates like `RPDS` that utilize the copy on write approch with the versioning to capture state changes.
Lock free data structure are specilized version of data structure that are vey challenging to implement .

# List 
Always remember that as a rust programmer must write the safe code .Unsafe is not a options.
The defining characterstic,storing things in a linear ,defined relationship with the each other helps to keep the track of stuff and find it later again.
 - `Linked list `
  To keep the track of a bunch of items,there is simple solutions:with the each entry in the list store a pointer to the next entry,if there is no next entry in stor null/nill/None and so on and keep to the pointer to the first item. This is what called singly list .
  Where each item is connected with the single link to the next.
  It is queue structure .
  It is very complex in rust as compare to other language .
    For real time implementation it is better to use std library for the rust.
 ```Rust
     // This is our node
// Breakdown of this code 
// struct = This simply defines that we are declaring the new  structure to hold the data.
// Not is name of the given structure.
// <> this indicates that it can hold the data different types of data here it can hold the data types of u128 bit.
// data is the name of the field of given struct which can hold the u128 unsigned but integer.
// next is another field name of same struct but its lil bit complex.
     //Option<> This is optional value it can be some vale too
     //Box<Not<u128>> This is boxed pointer to Not<u128>.
        // Box is a smart pointer that manages the memory location
        //Not<u128> refers back to the structer itself,allowing us to create linked list.
// This is our node
struct Not <u128> {
    data : u128,
    next: Option<Box<Not<u128>>>,
}

fn main (){
  
  }
   ```

   // This is pure implementation of linked list 
```Rust

type pointer= Option<Box<Node<u128>>>;

struct Node <u128> {
    data :u128,
    next :pointer,
}


struct linklist {
    head:pointer,
}

impl linklist {
 fn new ()->linklist {
    linklist {
        head:None,
    }
 }
fn remove (&mut self)-> Option<u128>{
match self.head.take() {
     Some (previous_head) => {
 self.head = previous_head.next;
 Some(previous_head.data)
     }
     None=>None,
    
}
}
 fn add (&mut self,data:u128){
let previous_head = self.head.take();
let new_head= Some(Box::new(Node{
    data:data,
    next:previous_head,
}));
self.head =new_head
 }



fn print (&self){
    let mut list =&self.head;
    while !list.is_none() {
        println!("{:?}",list.as_ref().unwrap().data);
        list=&list.as_ref().unwrap().next;
        
    }

}
}



fn main (){
let mut list = linklist::new();
list.add(45);
list.add(4509);
list.add(45008);
list.add(450007);
list.add(4500006);
list.add(45000005);
list.add(45000004);
list.add(450000003);
list.add(4500000002);
list.add(45000000001);
list.remove();
list.remove();
list.remove();
list.print();



}
```
   # Singly linked list 
   - A singly linked list is a linear data structure in which elements are stored in nodes, and each node points to the next node in the list. It is called a singly linked list because each node has a single link to the next node. In a singly linked list, you can traverse the list only in one direction, from the head (the first node) to the tail (the last node).
   - Travesre means visting each elements in the list to perform the operation.
   - Push means to insert at the end
  - Pop meand to remove from the begining 

# Dynamic array 
Array are one of the common way to store the data in the sequence order.However it lack the fundamental feature of list:expansion,Array are efficient because they are fixed-sized of container of length n where every element has the an equal size.Thus every element can be reached by calculating the address to jump to the using the simple formula `start_address +n*element size` 
Array are CPU cache-frinedly

# Tress
 
 # Binary Tree
```Rust
#[derive(Debug)]

struct  TreeNode<T> {
    val :T,
    left :Option<Box<TreeNode<T>>>,
    right :Option<Box<TreeNode<T>>>
}
impl <T> TreeNode<T> {
    pub fn new (val :T ) -> Self {
        TreeNode {
            val,
            left:None,
            right:None,
        }
    }
    pub fn insert(&mut self ,val :T ){
        self.left =Some(Box::new(TreeNode::new(val)))
    }
}

fn main (){
    let mut my_tree = TreeNode::new(785);
    println!("{:?}",my_tree);
    my_tree.insert(78564);
    println!("After insert {:?}",my_tree);
}
```





















