# Basic terminology 
 Operator is symbol which is use to mainpulate the data in the code.
 Operand is the data which is easily manipulated with the help of opearator .
 1 stand for the true .
 0 stand for the false.
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







