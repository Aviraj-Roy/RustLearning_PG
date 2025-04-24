//using reference to remove complexity of ownership, action called borrowing
fn main() {
    let mut s = String::from("Aviraj Roy");

    let len = calculate_len(&s);
    println!("(Immutable)->The len of {s} is {len}");

    let len = calculate_length(&mut s);
    println!("(Mutable)->The len of {s} is {len}");

    //let reference_to_nothing = dangle();
}

//s:String -> moving
//s:&String -> borrowing
fn calculate_len(s: &String) -> usize {
    //can't mutate the string 's' bcoz references r by default immutable
    s.len()
}

//passing a mutable reference to enable mutation
//Note:- there can be only 1 "mutable reference" of a var. >1 gives error
//altho can be used using the feature of scope
fn calculate_length(s: &mut String) -> usize {
    s.push_str(" says Hello!");
    s.len()
}

//multiple mutable & immutable ref cant be used consecutively, without "println!" statement in between

//dangling pointer—a pointer that references a location in memory that may have been given to someone else—by freeing some memory while preserving a pointer to that memory
/*fn dangle() -> &String {
    let s = String::from("hello");

    &s
}*/
