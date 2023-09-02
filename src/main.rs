mod m1_eurms;
mod m2_struct;
mod m3_traits;
mod m4_polymorphism;
mod m5_lifetime;
mod m6_patterns;
mod m7_async;
mod m8_collections;
mod m9_decl_marcos;
mod m10_proc_macros;
mod m11_smart_pointers;
mod m12_concurrency;

const OUR_COURSE: &str = "Rust Programming Language";


#[tokio::main]
async fn main() {
    // println!("Hello, world! {}", OUR_COURSE);

    // let x: i32;
    // x = 5;
    // println!("The value of x is: {}", x);

    // let y: i32 = 4;

    // println!("The value of y is: {}", y);

    // for i in 0..=y {
    //     println!("The value of i is: {}", i);
    //     if i != 4 {
    //         println!("The value of i is not 4");
    //     } else {
    //         println!("The value of i is 4");
    //     }
    // }

    // let freezing_temp: f64 = -2.4;

    // let my_inits: [f32; 10] = [2.0; 10];
    // println!("The value of my_inits is: {:?}", my_inits);
    
    // let my_floats_new: [f32; 10] = my_inits.map(|x| {
    //     x * 2.0
    // });
    // println!("The value of my_floats_new is: {:?}", my_floats_new);
    // println!("The value of my_inits is: {:?}", my_inits);

    // let myName: &str = "Rust";

    // let dynamic_string: String = String::from("Rust Course");
    // println!("The value of dynamic_string is: {}", dynamic_string);
    // println!("The value of dynamic_string is: {:p}", &dynamic_string);

    // let str_slice: &str = &dynamic_string[0..5];
    // println!("The value of str_slice is: {:p}", &str_slice);

    // let mut chars: Vec<char> = Vec::new();
    // chars.insert(0, 'R');
    // chars.insert(1, 'u');
    // chars.insert(2, 's');
    // chars.push('t');

    // dbg!(&chars);

    // let removed_char: char = chars.pop().unwrap();

    // dbg!(&chars);

    // chars.iter().for_each(|c: &char| {
    //     println!("The value of c is: {}", c);
    // });
    
    //  let char_again: Vec<char> = vec!('R','u','s','t');
    //     println!("{:?}", char_again);
    
    // let collected: String = char_again.into_iter().collect();
    // dbg!(collected);

    // let num: i32 = 5;
    // let add_num = |x: i32| x + num;
    // let new_num: i32 = add_num(5);
    // dbg!(new_num);

    // println!("Big Number is {}", 98_222_222);
    // println!("Hex is {}", 0xff);
    // println!("Octal is {}", 0o77);
    // println!("Binary is {}", 0b1111_0000);
    // println!("Byte is {}", b'A');

    // let text: &str = r#"{"Halo" : "Great"}"#;
    // println!("Text is {}", text);

    // let a: u8 = 0b_1010_1010;
    // println!("a is {}", a);
    // println!("a binary is {:b}", a);
    let mut a: &str = "Hello";
    let mut c: String = String::from("Hello world");

    c.push('!');
    let b = a;
    println!("The value of a is: {}", c);
}
