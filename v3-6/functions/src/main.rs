fn main() {
    println!("Hello, world!");
    let num = 5;
    my_function(num, true);
    let y = add(5, 6);
    println!("Sum = {y}");
    print!("The num ");
    is_event(num);
    print!("The sum ");
    is_event(y);
}

fn my_function(x: i32, y: bool) {
    println!("Hello from my_function, printing passed value = {x}. \nPrinting success? = {y}");
    let y = {
        let x = 3;
        x + 1
    };
    println!("{y}");
}

fn add(x: i32, y: i32) -> i32 {
    x + y //no need to use return keyword, statement becomes return statement when no ';'
}

fn is_event(x: i32) -> bool {
    if x % 2 == 0 {
        println!(" is even");
        return true; //since this is not last statement, need to use 'return' - called early return
    }
    println!("is not even");
    false
}
