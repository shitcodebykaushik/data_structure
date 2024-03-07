fn main(){
    let mut array  : [i32;7]=[0;7];
    array[1]=5;
   
    array[5]=6;
    array[2]=6;
    array[3]=6;
    array[2]=6;
    array[0]=7;
   assert_eq!([5,6,6,0,6,],&array[1..]);
   let x=array;
    //for x in array {
        print!("{}",x);
    }
