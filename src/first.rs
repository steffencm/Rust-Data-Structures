use std::mem;

pub struct List{
    head: Link,
}


enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}


impl List{

    pub fn new() -> Self {
        List{ head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32){
        let new_node = Box::new(Node{
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32>{
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(boxed_node) =>{
                let node = *boxed_node;
                self.head = node.next;
                Some(node.elem)
            }
        }
    }

}

impl Drop for List{
    fn drop(&mut self){
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        // while let == "do this until the pattern doesn't match"
        while let Link::More(mut boxed_node) = cur_link{
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
            //Boxed node goes out of scope and gets dropped here
            //but its Node's next field has been set to Link:: Empty
            //so no unbounded recursion happens here
        };
    }
}

mod test {
    #[test]
    fn basics() {
        use super::List;
        let mut list = List::new();

        //Assert empty list returns None on pop
        assert_eq!(list.pop(), None);

        //Popluate the list
        list.push(1);
        list.push(2);
        list.push(3);

        //Check normal list removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        //Push some More
        list.push(4);
        list.push(5);

        //And check removal again
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        //Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
