#[derive(Debug)]
enum Message{
    Quit,
    Move{x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quit"),
        Message::Move{x, y: new_name} => println!("Move to x: {}, new_name: {}", x, new_name),
        Message::Write(text) => println!("Write: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change color to r: {}, g: {}, b: {}", r, g, b),
    };
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_enums() {
        let msg: Message = Message::Quit;
        let my_colours: Message = Message::ChangeColor(0, 0, 0);
        let my_move: Message = Message::Move{x: 10, y: 100};
        let my_write: Message = Message::Write("Hello World!".to_string());
        process_message(my_write);
    }

    #[test]
    fn test_match_literals() {
        let number: i32 = 20;
        match number {
            1 => println!("One"),
            2 => println!("Two"),
            3 => println!("Three"),
            4 => println!("Four"),
            5 => println!("Five"),
            _ => println!("Something else"),
        };
    }
    #[test]
    fn tests_match_option() {
        let some_number: Option<u8> = Some(5);

        let res = if let Some(i) = some_number {
            println!("{}", i);
        }else{
            panic!("There was an problem")
        };
    //    let res = match some_number {
    //         Some(i) => i,
    //         None => {
    //             panic!("There was an problem")
    //         }
    //     };
        
        // dbg!(res);

        let some_res :Result<i32, &str> = Ok(5);
        let some_err :Result<i32, &str> = Err("There was an problem");

        let res = match some_res {
            Ok(i) => i,
            Err(e) => panic!("{}", e),
        };

        // dbg!(res);
    }

    #[test]
    fn tests_match_guard(){
        let pair = (2, -2);
        match pair {
            (x,y) if x == y => println!("These are twins"),
            (x,y) if x + y == 0 => println!("Antimatter, kaboom!"),
            (_, y) if y == 2 => println!("Second one is 2"),
            _ => println!("No correlation..."),
        };
    }

    #[test]
    fn tests_match_struct(){
        struct Location {
            x: i32,
            y: i32,
        }
        let location = Location{x: 1, y: 0};

        match location {
            Location{x, y}  if y == 0 => println!("Y is on the axis"),
            Location{x:0, y} => println!("X is on the axis"),
            Location{x, y} => println!("X and Y are not on the axis"),
        };
    }

}