#[derive(Debug)]
enum CarColour {
    Red,
    Green,
    Blue,
    Sliver
}

#[derive(Debug)]
enum GivenResult<T, E> {
    Ok(T),
    Err(E)
}

fn handle_car_colour() -> CarColour {
    let my_car_colour: CarColour = CarColour::Red;
    my_car_colour
}

fn check_under_five(num_check: u8) -> GivenResult<u8, String> {
    if num_check < 5 {
        GivenResult::Ok(num_check)
    } else {
        GivenResult::Err("Not under 5!".to_string())
    }
}

#[derive(Debug)]
enum GivenOption<T> {
    None,
    Some(T)
}

fn remainder_zero(num_check: f32) -> GivenOption<f32> {
    let remainder: f32 = num_check % 10.0;
    if remainder != 0.0 {
        GivenOption::Some(remainder)
    } else {
        GivenOption::None
    }
}
fn remainder_zero_built_in(num_check: f32) -> Option<f32> {
    let remainder: f32 = num_check % 10.0;
    if remainder != 0.0 {
        Some(remainder)
    } else {
        None
    }
}

fn check_under_five_built_in(num_check: u8) -> Result<u8, String> {
    if num_check < 5 {
        Ok(num_check)
    } else {
        Err("Not under 5!".to_string())
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_enums() {
        let car_colour: CarColour = handle_car_colour();
        dbg!(&car_colour);
        let is_ok: Result<u8, String> = check_under_five_built_in(7);
        dbg!(&is_ok);
        let remainder: Option<f32> = remainder_zero_built_in(12.2);
        dbg!(&remainder);
    }
}