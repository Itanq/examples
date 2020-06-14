use link_list::List;

fn main() {
    let mut list = List::new();
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);
    list.push_front(4);

    println!("{:?}->{:?}->{:?}->{:?}->{:?}",
             list.pop_front(),
             list.pop_front(),
             list.pop_front(),
             list.pop_front(),
             list.pop_front());

}
