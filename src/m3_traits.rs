trait Attacker {
    fn choose_style(&self) -> String;
   
}

#[derive(Debug)]
enum Character {
    Warrior,
    Archer,
    Wizard
}

impl Attacker for Character {
    fn choose_style(&self) -> String {
        match self {
            Character::Warrior => "Melee".to_string(),
            Character::Archer => "Ranged".to_string(),
            Character::Wizard => "Magic".to_string(),
        }
    }
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_traits() {
        let my_character: Character = Character::Warrior;
        let choosen_style: String = my_character.choose_style();
        dbg!(choosen_style);
    }

}