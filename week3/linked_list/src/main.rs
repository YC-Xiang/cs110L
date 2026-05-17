use linked_list::{ComputeNorm, LinkedList};

pub mod linked_list;

fn test_clone() {
    let mut original = LinkedList::new();
    original.push_front(3);
    original.push_front(2);
    original.push_front(1);

    let mut cloned = original.clone();

    assert!(original == cloned);
    assert_eq!(original.get_size(), 3);
    assert_eq!(cloned.get_size(), 3);

    cloned.pop_front();
    assert_eq!(cloned.get_size(), 2);
    assert_eq!(original.get_size(), 3);
    assert!(original != cloned);
}

fn test_partial_eq() {
    let empty_a = LinkedList::<i32>::new();
    let empty_b = LinkedList::new();
    assert!(empty_a == empty_b);

    let mut a = LinkedList::new();
    a.push_front(2);
    a.push_front(1);

    let mut b = LinkedList::new();
    b.push_front(2);
    b.push_front(1);
    assert!(a == b);

    let mut c = LinkedList::new();
    c.push_front(2);
    c.push_front(99);
    assert!(a != c);

    let mut shorter = LinkedList::new();
    shorter.push_front(1);
    assert!(a != shorter);
}

fn test_compute_norm() {
    let empty = LinkedList::<i32>::new();
    assert_eq!(empty.compute_norm(), 0.0);

    let mut single = LinkedList::new();
    single.push_front(5);
    assert_eq!(single.compute_norm(), 5.0);

    let mut triple = LinkedList::new();
    triple.push_front(5);
    triple.push_front(12);
    triple.push_front(0);
    assert!((triple.compute_norm() - 13.0).abs() < f64::EPSILON);
}

fn main() {
    test_clone();
    test_partial_eq();
    test_compute_norm();
    println!("Clone, PartialEq, and ComputeNorm tests passed");

    let mut list: LinkedList<u32> = LinkedList::new();
    assert!(list.is_empty());
    assert_eq!(list.get_size(), 0);
    for i in 1..12 {
        list.push_front(i);
    }

    println!("{}", list);
    println!("list size: {}", list.get_size());
    println!("top element: {}", list.pop_front().unwrap());
    println!("{}", list);
    println!("size: {}", list.get_size());
    println!("{}", list.to_string()); // ToString impl for anything impl Display
}
