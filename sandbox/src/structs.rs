// Structs - create custom data types

//Traditional structure
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Tuple Struct

struct ColorTupel(u8, u8, u8);

// Struct with functions
struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    //constructor
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    // get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };

    c.red = 200;

    println!("Color {} {} {}", c.red, c.blue, c.green);

    let mut new_c = ColorTupel(255, 0, 0);

    new_c.2 = 20;

    println!("{:?}", (new_c.0, new_c.1, new_c.2));

    let mut p = Person::new("sachin", "singh");

    println!("{}", p.full_name());

    p.set_last_name("bhadoriya");

    println!("person {} {}", p.first_name, p.last_name);

    println!("{:?}", p.to_tuple())
}
