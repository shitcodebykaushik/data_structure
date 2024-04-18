// This is node 
type pointer = Option<Box<Node<u128>>>;
  pub struct Node <u128> {
  data :u128,
  next:pointer,
 }
 // This is defination for the linked list and it is the process of declaring the structure,properties

 struct linklist {
    head:pointer,
 }
// impl linklist {} This line declares the implementation method  associated with the linklist
// fn new(): This is a static method associated with the LinkedList struct.
// -> LinkedList: This indicates that the method returns an instance of LinkedList.


impl linklist {
    fn new () -> linklist {
// This is method body which is creating a empty linked list 
        linklist {
            head :None,
        }
    }
   
    // Every function which we need to implement need to be declared here
    fn add (&mut self,data:u128){
        let previous_head = self.head.take();
        let new_head= Some(Box::new(Node{
            data:data,
            next:previous_head,
        }));
        self.head =new_head
         }

         // Here we are defining the method for the print implementation
         fn print (&self){
            let mut list =&self.head;
            while !list.is_none() {
                println!("{:?}",list.as_ref().unwrap().data);
                list=&list.as_ref().unwrap().next;
                
            }
        
        }
}


fn main (){
  let mut element =linklist::new();
  element.add(8);
  element.add(80);
  element.add(800);
 element.print();
  
  
}
