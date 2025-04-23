fn main() {
    //stack
    let num = 10;
    let result = add(num);

    //heap
    let name = String::from("Aviraj Roy");
    takes_ownership(name);
    let s = gives_ownership();

    println!("Num is {num} and result = {result}");

    //println!("Value of name is {name}");
    //doesnt work bcoz owner of name gets changed to the func - takes_ownership

    println!("S = {s}");

    let _s2 = takes_and_gives_back(s);
    //println!("S2 = {s2}");

    let s = String::from("Aviraj Roy");
    let (s, l) = calculate_len(s);
    println!("The len of {s} is {l}");
}

fn takes_ownership(s: String) {
    println!("Inside ownership {s}");
} //deletes value of name after the function call

fn gives_ownership() -> String {
    let s = String::from("This is a string from gives ownership");
    s
}

//main gist give & tack func
fn takes_and_gives_back(s: String) -> String {
    println!("S in takes_and_gives_back {s}");
    s
}
fn add(x: i32) -> i32 {
    x + 10
}

//takes ownership, does the job & returns the ownership
fn calculate_len(s: String) -> (String, usize) {
    let res = s.len();
    (s, res)
}
