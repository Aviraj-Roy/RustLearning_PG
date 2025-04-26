struct User {
    active: bool,
    username: String, //taking ownership
    email: String,
    sign_in_count: u64,
}

fn main() {
    //the entire instance should be "mut" if req
    let mut user_1 = User {
        //instance of the above struct
        email: String::from("zapros2004@gmail.com"),
        username: String::from("Aviraj Roy"),
        active: true,
        sign_in_count: 0,
    };
    println!("The name of the user is {}", user_1.username);

    user_1.username = String::from("MrYor");
    println!("The new username of the user is {}", user_1.username);

    //making new user using build_user func
    let user_2: User = build_user(String::from("A.Roy"), String::from("roy@email.com"));
    println!("The name of the new user is {}", user_2.username);

    let user_3 = User {
        active: user_1.active,
        username: user_1.username,
        email: String::from("john@email.com"),
        sign_in_count: user_1.sign_in_count,
    };

    let user_4 = User {
        email: String::from("john@email.com"),
        ..user_1 //copying other details frm user_1
                 //user_1 can't be used after this, everything moved frm user_1
    };
}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 0,
    }
}
