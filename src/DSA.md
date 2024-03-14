# Basic terminology 
 Operator is symbol which is use to mainpulate the data in the code.
 Operand is the data which is easily manipulated with the help of opearator .
 1 stand for the true .
 0 stand for the false.
 In programming when you use the term `inclusive` it simply means you are considiring the start and end point of the range .
 Statically generated means that code or data  which is generated at the compile time rather then runtime.
`Problem` It is a question to which we seek an answer. 
Algorithm is the step by procedure which can solve any instances of a problem. 
 Sorting means the process of arranging of elements in specific order.Typically based on some predefined criteria.
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




`Fixed size Array `

`DYNAMICALLY TYPED `
Here size is not known at the compile time .
In this size grow automatically whenever there is need of it .
In this the insertion of data is at the last .
In rust it exist behind the pointer.
Pointer of `Dynamicaaly size type` become wide pointer because it contain the information and pointer both.
`HEAP Array` 
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
 # EExchange sort 
 Exchange sort" is a term that typically refers to sorting algorithms where elements are compared and swapped to arrange them in a specific order, such as ascending or descending. The most well-known exchange sort algorithm is the "Bubble Sort
 # Sequential search Versus Binary Search
    Binary search is efficiant technique of searching and it is quite complex ,as it works on the concept of divide and conquer.
    Time comlexity is O(log n).
    Sequential search is simple technique of searching and it is easy to implement ,it is iterative in nature. Time complexity is O(n).

#  Analysis of Algorithm 
To determine the how efficincy an algorithm work.
  # Complexity  Analysis 
 # docker image ls 
 
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
  It worjs on the divide and conquer factor .
  It was developed by Hoare (1962). It is similiar to Mergesort .
  The main fundamental of quicksort is that the array is partitioned by placing all items smaller then the pivot items before that items and all items larger then the pivot items after that.For simplicity we can make any itmem as pivot itmes else we sometime chosse first itme as pivot items.
  Example :- 15,22,13,27,12,10,20,25
             Here we have selected 15 as pivot items and now we have arranged the array in smaller one 
             10,13,12 [All smaller array ] `15` 22,27,20,25 [All larger array]. Now we can sort this accoring to our need.


              






