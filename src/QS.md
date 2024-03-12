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
