fn main() {
    let number = 3;

    if number < 5 {
        println!("condition true");
    } else {
        println!("condition false");
    }

    let condition = true;
    let mut number: u8 = if condition { 5 } else { 6 };
    println!("Value = {number}");

    loop {
        //infinite loop
        println!("running continously");
        number += number;
        //use cargo run --release to execute this, normal cargo run will panick bcoz of overloading
        if number == 0 {
            break;
        }
    }

    result_counter();
    result_num();
    mut_loop();
    while_loop();
    for_loop();
}

fn result_counter() {
    println!("New Function for break & continue");
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");
}

fn result_num() {
    println!("New Function for returning thrgh break");
    let mut num = 1;
    let result = loop {
        println!("Value of number is {num}");

        if num == 10 {
            break 70;
        }
        num += 1;
    };
    println!("This is an end - {result}");
}

fn mut_loop() {
    println!("New Function for multiple loops - condition unknown");
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}

fn while_loop() {
    println!("New Function for while - condition known");
    let mut num = 5;
    while num != 0 {
        println!("{num}");
        num -= 1;
    }
    println!("LIFTOFF!");
}

fn for_loop() {
    println!("New Function for for - condition known");
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    println!("printing values using .len() & while -> ");
    while index < a.len() {
        println!("the value is: {}", a[index]);
        index += 1;
    }
    println!("LIFTOFF!");

    println!("printing values using just 'for'");
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }

    println!("printing no.s in range (exclusive upper range)");
    for x in 1..10 {
        //making upper value exclusive
        println!("x is {x}");
    }

    println!("printing no.s in range (inclusive upper range)");
    for x in 1..=10 {
        //making upper value inclusive
        println!("x is {x}");
    }

    println!("reverse printing using .rev()");
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
