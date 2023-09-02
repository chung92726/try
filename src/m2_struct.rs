#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    fn change_username(&mut self, new_username: &str) {
        self.username = new_username.to_string();
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_structs() {
        let mut user1: User = User {
            username: String::from("user1"),
            sign_in_count: 1,
            active: true,
            email: String::from("ccc@gmail.com"),};
            let mut user2: User = User {
            username: String::from("user3"),
            sign_in_count: 3,
            active: false,
            email: String::from("aaa@gmail.com"),};
        user2.change_username("user4");
        user1.change_username("user2");
    dbg!(user2);
    }

}