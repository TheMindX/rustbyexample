
use std::fmt;
#[derive(Debug)]
pub enum Linked_node<T:ToString + fmt::Display> {
    last,
    cons{val: T, next:Box<Linked_node<T> >},
}

impl <T:ToString + fmt::Display> Linked_node<T>{
    // add code here
    pub fn new() -> Linked_node<T>{
        Linked_node::last
    }

    pub fn prepend(self, v:T) -> Linked_node<T> {
        Linked_node::<T>::cons{val:v, next:Box::new(self)}
    }
}

impl <T:ToString + fmt::Display> ToString for Linked_node<T>{
	fn to_string(&self) -> String{
        match *self {
		    Linked_node::last => "last".to_string(),
		    Linked_node::cons{val:ref v,  next:ref n} => format!("({}, {})", v, n.to_string() ),
		}
	}
}