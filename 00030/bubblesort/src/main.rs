fn main() {
    let mut list: Vec<i32> = vec!(4, 2, -2, 0);
    bubblesort(&mut list);
    println!("{:?}", list);
}

// TODO
fn bubblesort() {
}

#[test]
fn test_bubblesort() {
    let mut list: Vec<i32> = vec!(7,2,0,-8,9,1,5,2);
    let expected: Vec<i32> = vec!(-8,0,1,2,2,5,7,9);
    bubblesort(&mut list);
    assert_eq!(expected, list);
}