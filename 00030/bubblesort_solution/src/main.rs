fn main() {
    let mut list: Vec<i32> = vec!(4, 2, -2, 0);
    bubblesort(&mut list);
    println!("{:?}", list);
}

fn bubblesort(list: &mut Vec<i32>) {
    let mut fin = false;
    while !fin {
        let mut did_swap = false;
        for i in 0..list.len() {
            if list.len() > i+1 {
                if list[i] > list[i+1] {
                    let x = list[i+1];
                    let y = list[i];
                    list[i] = x;
                    list[i+1] = y;
                    did_swap = true;
                }
            }
        }
        if !did_swap { fin = true; }
    }
}

#[test]
fn test_bubblesort() {
    let mut list: Vec<i32> = vec!(7,2,0,-8,9,1,5,2);
    let expected: Vec<i32> = vec!(-8,0,1,2,2,5,7,9);
    bubblesort(&mut list);
    assert_eq!(expected, list);
}