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