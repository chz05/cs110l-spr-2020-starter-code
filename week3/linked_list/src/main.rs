use linked_list::LinkedList;
pub mod linked_list;

fn main() {
    let mut list: LinkedList<u32> = LinkedList::new();
    let mut compare_list: LinkedList<u32> = LinkedList::new();
    assert!(list.is_empty());
    assert_eq!(list.get_size(), 0);
    for i in 1..12 {
        list.push_front(i);
    }
    for i in (1..12).rev() {
        compare_list.push_front(i);
    }
    println!("{}", list);
    println!("list size: {}", list.get_size());
    println!("top element: {}", list.pop_front().unwrap());
    println!("{}", list);
    println!("size: {}", list.get_size());
    println!("{}", list.to_string()); // ToString impl for anything impl Display
    let clone_list = list.clone(); // Clone trait
    println!("Cloned list: {}", clone_list);
    assert_eq!(list.to_string(), clone_list.to_string());
    
    println!("--- String LinkedList ---");

    let mut string_list: LinkedList<String> = LinkedList::new();
    string_list.push_front("hello".to_string());
    string_list.push_front("world".to_string());
    println!("{}", string_list);
    println!("size: {}", string_list.get_size());
    println!("top element: {}", string_list.pop_front().unwrap());
    println!("{}", string_list);
    println!("size: {}", string_list.get_size());
    println!("{}", string_list.to_string());
    let clone_string_list = string_list.clone();
    println!("Cloned string list: {}", clone_string_list);
    // If you implement iterator trait:
    //for val in &list {
    //    println!("{}", val);
    //}
}
