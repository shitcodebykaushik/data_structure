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



