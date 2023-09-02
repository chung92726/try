

#[cfg(test)]
mod tests {

    use my_proc_macro::function_to_string;

    const OUTPUT: &str = "This is a test";

    #[function_to_string]
    fn some_function_for_ai_llm(_whatever_param: &str) {
        println!("{}", OUTPUT);
        println!("This is a test2");
    }

    #[test]
    fn tests_proc_macro(){

        let x: &str = some_function_for_ai_llm("whatever");
        dbg!(x);

    }
}