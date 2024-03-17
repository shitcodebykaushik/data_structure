# For the last bit to be 1 or not .
 If the given qs fullfil this condition the we can say that yes its last number is 1.
 `(n&1)!=0`
 ```Rust
 fn main(){
 let c=6;
 if c&1 !=0{
    print!("Yes")
 } else {
    print!("No ")
 }
 

}
 ```
 ```
[This need to fix
 fn kth_value(n:u32,k:u32)->bool{
   let mask=1<<k;
    n&mask  !=0

}
fn main(){
let n: u32=5;
let k :u32=1;
let d: bool=kth_value( n,k);
println!("The value is  {}",d)]
fn main (){
    fn count( mut n :i32 ){
    let count = 0;
    while n!=0{
        n&=n-1;
      count+1;
    }

    }
    let v: ()=count(5);
    print!("{}",v);
}
```


# Brian Kernighan Algorithm for  to count set Bit




  
In bit we say `set` to typicaly to particular bit in binary whose value at particular index should be 1.
For example set at 1 means in 5 is the the value at index 1 should be 1.


# Algorithm 1.1
  ## Sequential Search 
  Is the key x in the array S of n keys?

```Rust

fn sequential (arr: &[i32],traget : i32) -> Option<usize>{
    for (index,&value)in arr.iter().enumerate() {
        if value ==traget {
            return Some(index);
        }
    }
    None

}

fn main (){
  let arr1 = [1,2,3,4,5,6];
  let target2  =20;
  match  sequential(&arr1, target2) {
    Some(index) => println!("Target found at index : {}",index),
    None => println!("The target value is not found")


  }
  
  }

```
# Algorithm 1.2
 ## Add Array Members
 ```Rust
 fn add ( sum : &[i32]) -> i32 {
    let mut sum1 =0;
    for  &k3 in sum {
    sum1+=k3;
}
 sum1 
}
fn main (){
    let  k4 = [4,5,6];
    let sum3=add(&k4);

  print!("{}",sum3);
}
 ```
 # Algorithm 1.3
 ```Rust
 fn nondecreasing_sort (g1 : &mut[u128]){
 let n = g1.len();
 for i in 0..n {
    for j in 0..n-i-1 {
        if  g1[j]>g1[j+1]{
            g1.swap(j, j+1);
        }
    }
 }
} 
fn main (){
    let mut arr = [23,789,45];
    println!("The value befor sorting {:?}",arr);
    
    nondecreasing_sort(&mut arr);
    println!("The value befor sorting {:?}",arr);
}
 ```
 # Algorithm 1.4
  ## Matrix Multiplication
   Problem determine the product of two n*n matrics 

   
  # Algorithm 1.5
  -  Binary Search
   Problem : Determine wheather x is in the sorted array S of 'n' keys 
   Output: Location of x in S (0 if x is not in S ).

 ```Rust
 fn binary_search (arr : &[i32], target : i32) -> Option<usize>{
    let mut low = 0;
    let mut high = arr.len () -1;
    while low <=high {
        let mid = low +(high -low) /2 ;

        if arr[mid] == target {
            return Some((mid)); 
        } else if arr[mid] <target {
            low = mid +1;
            
        } else {

            high =mid -1;

        }
        }
         None
    
}
fn main (){
    let  a1 = [1,2,3,4,5];
    let d2 =40;
     match binary_search(&a1, d2) {
         Some(index) => println!( "{} found at the index {:?}", d2,a1),
         None => println!("{} not found in the array ",d2),
     }
}
 ```
 # Algorithm 1.6
 nth Fibonachi Term (Recursive)
 ``` Rust 
 fn fid ( n : usize  ) -> i128 {
if n <= 1 {
   n as i128
}else  {
      fid( n-1) + fid(n-2)
}
}
fn main (){
 let f = 12;
 println!( "Fibonacci number is {} : {:?}",f,fid(f))
}

 ```
 # Algorithm 2.2
  `Mergesort`
  Problem: Sort n keys in nondecreasing sequence.
  Inputs: positive integer n, array of keys S indexed from 1 to n.
  Outputs: the array S containing the keys in nondecreasing order
# Algorithm 2.6 Quicsort 
```Rust 
fn quick_sort (arr :&mut [i32]) {
   if arr.len() <= 1{
   return;
}

 let pivot_index = partition(arr);
 quick_sort(&mut arr[..pivot_index]);
 quick_sort(&mut arr[pivot_index +1..]);

}
 fn partition (arr : &mut [i32]) -> usize {
   let pivot_index = arr.len() /2;
   arr.swap(pivot_index, arr.len() -1 );

   let mut i =0;
   for j in 0..arr.len () -1 {
      if arr [j] <= arr[arr.len ()-1]{
         arr.swap(i, j);
         i+=1;
      }
   }
   arr.swap(i, arr.len() -1 );
   i

 }
 fn main (){
 let mut k3 = [45,2,8,79,25];
 quick_sort(&mut k3);
 println!("The sorted array id {:?}",k3);
 }
```
# Algorithm 2.8
 Strassen
```Rust
fn strassen_multiply(a: &[Vec<i32>], b: &[Vec<i32>]) -> Vec<Vec<i32>> {
   let n = a.len();
   assert!(n == b.len() && n.is_power_of_two());

   if n == 1 {
       return vec![vec![a[0][0] * b[0][0]]];
   }

   let m = n / 2;

   let mut a11 = vec![vec![0; m]; m];
   let mut a12 = vec![vec![0; m]; m];
   let mut a21 = vec![vec![0; m]; m];
   let mut a22 = vec![vec![0; m]; m];
   let mut b11 = vec![vec![0; m]; m];
   let mut b12 = vec![vec![0; m]; m];
   let mut b21 = vec![vec![0; m]; m];
   let mut b22 = vec![vec![0; m]; m];

   for i in 0..m {
       for j in 0..m {
           a11[i][j] = a[i][j];
           a12[i][j] = a[i][j + m];
           a21[i][j] = a[i + m][j];
           a22[i][j] = a[i + m][j + m];
           b11[i][j] = b[i][j];
           b12[i][j] = b[i][j + m];
           b21[i][j] = b[i + m][j];
           b22[i][j] = b[i + m][j + m];
       }
   }

   let s1 = strassen_multiply(&matrix_add(&a11, &a22), &matrix_add(&b11, &b22));
   let s2 = strassen_multiply(&matrix_add(&a21, &a22), &b11);
   let s3 = strassen_multiply(&a11, &matrix_sub(&b12, &b22));
   let s4 = strassen_multiply(&a22, &matrix_sub(&b21, &b11));
   let s5 = strassen_multiply(&matrix_add(&a11, &a12), &b22);
   let s6 = strassen_multiply(&matrix_sub(&a21, &a11), &matrix_add(&b11, &b12));
   let s7 = strassen_multiply(&matrix_sub(&a12, &a22), &matrix_add(&b21, &b22));

   let p1 = matrix_add(&matrix_sub(&matrix_add(&s1, &s4), &s5), &s7);
   let p2 = matrix_add(&s3, &s5);
   let p3 = matrix_add(&s2, &s4);
   let p4 = matrix_add(&matrix_sub(&matrix_add(&s1, &s3), &s2), &s6);

   let mut result = vec![vec![0; n]; n];
   for i in 0..m {
       for j in 0..m {
           result[i][j] = p1[i][j];
           result[i][j + m] = p2[i][j];
           result[i + m][j] = p3[i][j];
           result[i + m][j + m] = p4[i][j];
       }
   }
   result
}

fn matrix_add(a: &[Vec<i32>], b: &[Vec<i32>]) -> Vec<Vec<i32>> {
   a.iter()
       .zip(b.iter())
       .map(|(row_a, row_b)| row_a.iter().zip(row_b.iter()).map(|(x, y)| x + y).collect())
       .collect()
}

fn matrix_sub(a: &[Vec<i32>], b: &[Vec<i32>]) -> Vec<Vec<i32>> {
   a.iter()
       .zip(b.iter())
       .map(|(row_a, row_b)| row_a.iter().zip(row_b.iter()).map(|(x, y)| x - y).collect())
       .collect()
}

fn main() {
   let a = vec![
       vec![1, 2, 3, 4],
       vec![5, 6, 7, 8],
       vec![9, 10, 11, 12],
       vec![13, 14, 15, 16],
   ];

   let b = vec![
       vec![17, 18, 19, 20],
       vec![21, 22, 23, 24],
       vec![25, 26, 27, 28],
       vec![29, 30, 31, 32],
   ];

   let result = strassen_multiply(&a, &b);
   println!("Result: {:?}", result);
}

``` 
# Algorithm 2.9
Larger Integer Multiplication 
Problem: Multiply two large integers, u and v.
Inputs: large integers u and v.
Outputs: prod, the product of u and v.
```Rust

use num_bigint::BigInt;
use num_traits::One;
fn main (){
   let i = BigInt::parse_bytes(b"1111111111111111111111111111111",10).unwrap();
   let j = BigInt::parse_bytes(b"2222222222222222222222222222222",30).unwrap();
 let result =i*j;
 println!("Result :{}",result);
}

```
# The Binomial Coefficient 
# Arrays Playlist 
