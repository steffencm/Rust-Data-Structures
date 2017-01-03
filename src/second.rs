pub struct List<T>{
    head: Link<T>,
}


type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}


impl<T> List<T>{

    pub fn new() -> Self {
        List{ head: None }
    }

    pub fn push(&mut self, elem: T){
        let new_node = Box::new(Node{
            elem: elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T>{
        self.head.take().map( |boxed_node| {
            let node = *boxed_node;
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&mut self) -> Option<&T>{
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T>{
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }
}

impl<T> Drop for List<T>{
    fn drop(&mut self){
        let mut cur_link = self.head.take();
        // while let == "do this until the pattern doesn't match"
        while let Some(mut boxed_node) = cur_link{
            cur_link = boxed_node.next.take();
            //Boxed node goes out of scope and gets dropped here
            //but its Node's next field has been set to Link:: Empty
            //so no unbounded recursion happens here
        };
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> List<T>{
    fn into_iter(self) -> IntoIter<T>{
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T>{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item>{
        self.0.pop()
    }
}

mod test {
    use super::List;

    #[test]
    fn peek(){
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);

        list.push(1);
        list.push(2);

        assert_eq!(list.peek(), Some(&2));
        assert_eq!(list.peek_mut(), Some(&mut 2));

    }

    #[test]
    fn basics() {

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

    #[test]
    fn into_iter() {
        let mut list = List::new();

        list.push(1); list.push(2); list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
    }
}
