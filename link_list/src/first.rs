
pub struct List {
    head: Link
}

enum Link {
    Empty,
    More(Box<Node>)
}

struct Node {
    elem: i32,
    next: Link
}

impl List {
    pub fn new() -> Self {
        List {
            head: Link::Empty
        }
    }

    pub fn push_front(&mut self, elem: i32) {
        let node = Node {
            elem,
            next: std::mem::replace(&mut self.head, Link::Empty)
        };
        self.head = Link::More(Box::new(node));
    }

    pub fn pop_front(&mut self) -> Option<i32> {
        match std::mem::replace(&mut self.head, Link::Empty) {
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            },
            Link::Empty => None
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = std::mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut node) = cur_link {
            println!("drop node: {}", node.elem);
            cur_link = std::mem::replace(&mut node.next, Link::Empty);
        }
    }
}

mod test {

    use super::List;

    #[test]
    fn test() {
        let mut list = List::new();
        assert_eq!(list.pop_front(), None);

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        list.push_front(4);

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
}