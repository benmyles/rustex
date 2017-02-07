// Resources:
//   - http://rustbyexample.com/primitives/tuples.html

fn main() {
    let answer = multiply_fractions((1, 5), (2, 3));
    println!("1/5 * 2/3 = {}/{}", answer.0, answer.1);
}

// TODO
fn multiply_fractions(f1: (i32, i32), f2: (i32, i32)) -> (i32, i32) {
    (f1.0 * f2.0, f1.1 * f2.1)
}

#[test]
fn test_multiply_fractions() {
    assert_eq!(
        (2, 15),
        multiply_fractions((1, 5), (2, 3))
    );

    assert_eq!(
        (1, 20),
        multiply_fractions((1, 4), (1, 5))
    );
}