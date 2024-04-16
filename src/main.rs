 // Append entrie from the end.
 // Removing entries from the front .

use std::cell::RefCell;
use std::rc::Rc;
 type SingleLink = Option<Rc<RefCell<Node>>>;
 // Rc<RefCell<Node>> Storing each data here helps in internall mutability. It is crucial while executing operation on the list.


 //# [derive(Clone)]
struct  Node {
    value :String,
    next :SingleLink
}
 impl Node {
    fn new (value : String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node{
            value : value,
            next: None,
        }))
    }
 }

struct TransactionLog {
    head: SingleLink,
    tail:SingleLink,
    pub length :u64
}

impl TransactionLog {
    pub fn new_empty () -> TransactionLog {
        TransactionLog {head:None,tail:None,length:0}
    }
    pub fn append (&mut self ,value :String) {
        let new =Node::new(value);
        match self.tail.take() { 
            Some(old)=>old.borrow_mut().next =Some(new.clone()),
            None => self.head=Some(new.clone())
            
        };
        self.length +=1;
        self.tail =Some(new);
        
    }
}


fn main (){
    
}