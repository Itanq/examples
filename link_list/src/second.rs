
#[derive(Debug)]
pub struct List<T> {
    head: Link<T>
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>
}

impl<T> List<T> {

    pub fn new() -> Self {
        List{ head: None }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter{
            next: self.head.as_ref().map(|node| &**node )
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut{
            next: self.head.as_mut().map(|node| { &mut **node })
        }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn push_front(&mut self, elem: T) {
        let node = Node{
            elem,
            next: self.head.take()
        };
        self.head = Some(Box::new(node));
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node|{
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|n| &**n);
            &node.elem
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|mut node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur = std::mem::replace(&mut self.head, None);
        while let Some(mut node) = cur {
            cur = std::mem::replace(&mut node.next, None);
        }
    }
}

mod test {

    use super::List;

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        assert_eq!(list.pop_front(), None);

        list.push_front("hello");
        list.push_front("hi");
        list.push_front("world");

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut "world"));
        assert_eq!(iter.next(), Some(&mut "hi"));
        assert_eq!(iter.next(), Some(&mut "hello"));
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        assert_eq!(list.pop_front(), None);

        list.push_front("hello");
        list.push_front("hi");
        list.push_front("world");

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&"world"));
        assert_eq!(iter.next(), Some(&"hi"));
        assert_eq!(iter.next(), Some(&"hello"));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);

    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        list.peek_mut().map(| v| {
            *v = 10 * *v;
        });

        assert_eq!(list.peek(), Some(&30));
    }

    #[test]
    fn test() {
        let mut list = List::new();
        assert_eq!(list.pop_front(), None);

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        list.push_front(4);

        println!("list:{:?}", list);

        assert_eq!(list.pop_front(), Some(4));
        assert_eq!(list.pop_front(), Some(3));

        list.push_front(5);
        list.push_front(6);

        assert_eq!(list.pop_front(), Some(6));
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(2));

        list.pop_front();
        assert_eq!(list.pop_front(), None);

    }

    #[test]
    fn test_str() {
        let mut list = List::new();
        assert_eq!(list.pop_front(), None);

        list.push_front("hello");
        list.push_front("world");
        list.push_front("rust");

        println!("list:{:?}", list);

        assert_eq!(list.pop_front(), Some("rust"));
        assert_eq!(list.pop_front(), Some("world"));

        list.push_front("program");

        assert_eq!(list.pop_front(), Some("program"));
        assert_eq!(list.pop_front(), Some("hello"));

        assert_eq!(list.pop_front(), None);

    }

}