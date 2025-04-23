fn main() {
    let s = "Hello World!"; //immutable

    {
        let x = "Hey frm X"; // X scope starts
        println!("X = {x}");
    } //scope ends
      // drop(x) called implicitly here to destruct value of x, freeing memory

    // println!("{}", x); // cant access due to ownership scope
    println!("{}", s);

    //heap concept, mutable
    let mut s = String::from("hello"); //requesting memory, space unknown
    println!("S = {}", s);
    s.push_str(" world");
    println!("S = {}", s);

    let mut x = 5;
    let y = x; //value copied
    x = 10;
    println!("X is {x} & Y is {y}");

    let s1 = String::from("I am X"); //pointer
                                     //shallow copy
                                     //let s2 = s1; //tries to copy pointer, but doesnt coz it's expensive during runtime
                                     // drop tries to free both s1 & s2 when out of scope, but s2 would be a shallow copy of s1 (s1 moved to s2), leading to a double free error
    println!("S1 is - {s1}");
    //deep copy, explicitly done but still expensive
    let mut s2 = s1.clone(); //allocates a new memory in heap making s1 & s2 diff
    println!("S2 - {s2}");
    s2.push_str(" s2, after cloned & new str added");
    println!("S2 - {s2}");
} // drop(s) called implicitly here to destruct value of s, freeing memory
