//  conditional - used to check the conditon of something and act on it

pub fn run() {
    let age: u8 = 18;
    let check_id: bool = true;
    let knows_person_of_age: bool = true;
    // if else
    if age >= 21 && check_id || knows_person_of_age {
        println!("Bartender: What would you like to drink?");
    } else if age < 21 && check_id {
        println!("I will need an ID")
    } else {
        println!("This is not for you");
    }

    // no turnary , but a short hand

    //shorthand if
    let is_of_age = if age >= 21 { true } else { false };

    println!("{}", is_of_age);
}
