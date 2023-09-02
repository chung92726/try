#[cfg(test)]
mod tests {
    

    macro_rules! mad_skills {
        // ($x: expr) => {
        //     format!("{} is a mad skill", $x)
        // };
        ($x: ty) => {
            match stringify!($x) {
                "i32" => format!(" is a mad skill"),
                _ => format!(" is not a mad skill"),
            }
        }
    }

    macro_rules! mad_skills_2 {
        ($($x: expr), +) => {
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                )+
                temp_vec
            }
        };
    }

    #[test]
     fn test_marco(){

        let some_var  = mad_skills_2!(mad_skills_2!(1,2,3), mad_skills_2!(4,5,6));

        dbg!(some_var);
    }
    
}