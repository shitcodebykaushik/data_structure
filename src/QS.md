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
