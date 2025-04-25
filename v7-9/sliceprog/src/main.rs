fn main() {
    let s = String::from("Hello World");
    let res = find_first_word(&s);

    println!("For string - {s}, the Result is {res}");

    let res = first_word(&s);
    println!(
        "For string - {s}, the Result is {res} & length is {}",
        res.len()
    );
}

fn find_first_word(input: &String) -> usize {
    println!("Without Using Slice -> ");
    let s = input.as_bytes(); //converting to an array of bytes

    for (i, &item) in s.iter().enumerate() {
        // iter returns each element in a collection
        // enumerate wraps the result & returns each elemenet as a part of tuple
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word(input: &str) -> &str {
    println!("With Using Slice -> ");
    let s = input.as_bytes();

    for (i, &item) in s.iter().enumerate() {
        if item == b' ' {
            return &input[..i];
        }
    }
    &input[..]
}
