use std::collections::{HashMap, HashSet};



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
     fn test_hashmap(){

        let person_1 = "alice";
        let person_2 = "bob";
        let mut results_hm: HashMap<&str, u32> = HashMap::new();
        results_hm.insert(person_1, 55);
        results_hm.insert(person_2, 77);

        let test_res = results_hm.get(person_1);
        dbg!(test_res.unwrap());

        
    }
    #[test]
    fn test_hashset () {
        let mut names_hs: HashSet<&str> = HashSet::new();
        let person_1 = "alice";
        names_hs.insert("alice");
        names_hs.insert("bob");
        names_hs.insert("alice");

        println!("{:p}", person_1);

        if names_hs.contains("alice") {
            println!("Alice is in the set");
        }
    }
}
