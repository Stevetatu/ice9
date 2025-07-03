struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
fn main(){
    let mut user1: User = User{
        email: String::from("Someone@example.com"),
        username: String::from("SomeUsername123"),
        active: true,
        sign_in_count: 1,
    };

    let _name: String = user1.username;
    user1.username = String::from("Wallace12");
    let user2 = build_user(String::from("jane@example.com"), String::from("JaneDoe"));
    println!("User1: {}, Email: {}, Active: {}, Sign In Count: {}", user1.username, user1.email, user1.active, user1.sign_in_count);
}

fn build_user(email: String, username: String) -> User{
    User{
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
