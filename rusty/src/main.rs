// fn name_print(first: &str, second: &str) {
//     println!("{}", first);
//     println!("{}", second);
// }

// fn add(a: i32, b: i32) -> i32 {
//     a + b
// }

// fn hgb(greet: bool)  {
//     if greet {
//         println!("hello");
//     } else {
//         println!("goodbye");
//     }

// }

// fn five(num: i32) {
//     if num > 5 {
//         println!(">5");
//     } else if num == 5 {
//         println!("=5");
//     } else {
//         println!("<5");
//     }
// }
//
// fn matchtest(my_bool: bool)  {
//     match  my_bool {
//         true => println!("it's true"),
//         false => println!("it's false"),

//     }

// fn what_num(num: i32)  {
//     match num {
//         1 => println!("one"),
//         2 => println!("two"),
//         3 => println!("three"),
//         _ => println!("other"),
//     }
// }

// fn display_until(end: i32)  {
//     let mut iter = 1;
//     loop {
//         println!("{iter}");
//         if (iter == end) { break;}
//         iter = iter + 1;

//     }
// }

// fn countdown(mut from: i32)  {
//     // put mut before function arg to indicate its mutable inside, num put in doesn't need to be
//     // mut
//     while (from > 0) {
//         println!("{from}");
//         from = from - 1;
//     }
// }

// enum Colours {
//     blue,
//     green,
//     red,
//     orange,
// }

// fn print_colour(my_colour: Colours) {
//     match my_colour {
//         Colours::blue => println!("blue"),
//         Colours::green => println!("green"),
//         Colours::red => println!("red"),
//         Colours::orange => println!("orange"),
//     }

// }

// enum Flavour {
//     Strawberry,
//     Raspberry,
//     Choc,
//     Mint,
//     Matcha,
// }

// struct Drink {
//     flavour: Flavour,
//     volume: i32,
// }

// fn drink_details(my_drink: Drink) {
//     match my_drink.flavour {
//         Flavour::Strawberry => println!("You have {} of strawberry left",my_drink.volume),
//         Flavour::Raspberry => println!("You have {} of raspberry left",my_drink.volume),
//         Flavour::Choc => println!("You have {} of choc left",my_drink.volume),
//         Flavour::Mint => println!("You have {} of mint left",my_drink.volume),
//         Flavour::Matcha => println!("You have {} of matcha left",my_drink.volume),
//     }

// }

// fn is_big(my_bool: bool) {
//     match my_bool {
//         true => println!("it's big"),
//         false => println!("it's small"),
//     }

// }
// struct item {
//     quantity: i32,
//     id: i32,
// }

// fn disp_quant(my_item: &item) {
//     println!("The quantity is: {}", my_item.quantity );

// }
// fn disp_id(my_item: &item) {
//     println!( "The id is: {}", my_item.id);
// }

// #[derive(Debug)]
// struct Dimension {
//     x: f64,
//     y: f64,
//     z: f64,
// }
// struct Box {
//     dimension: Dimension,
//     weight: f64,
//     colour: Colour
// }
// impl Colour {
//     fn print(&self) {
//         match self {
//             Colour::Blue => println!("The box is blue!"),
//             Colour::Green => println!("The box is green!"),
//             Colour::Red => println!("The box is red!"),
//         }
//     }
// }
// impl Box {
//     fn new(weight: f64, colour: Colour, dimension: Dimension) -> Self {
//         Self {
//             weight,
//             colour,
//             dimension
//         }
//     }

//     fn print_box(&self) {
//         println!("Box has dimensions of {}, {}, {}", self.dimension.x,self.dimension.y,self.dimension.z);
//         println!("Box weighs {}",self.weight);
//         self.colour.print()

//     }

// }
// impl Dimension {
//     fn new(x:f64,y:f64,z:f64) -> Self {
//         Self {
//             x,
//             y,
//             z
//         }
//     }
// }
// enum Colour {
//     Blue,
//     Green,
//     Red
// }

// struct Person {
//     Age: i32,
//     Name: String,
//     FavCol: String,
// }

// let people = vec![
//     Person {
//         Age: 28,
//         Name: "Jamie".to_owned(),
//         FavCol: "Red".to_owned(),
//     },
//     Person {
//         Age:27,
//         Name: "Bob".to_owned(),
//         FavCol: "Green".to_owned(),
//     },
//     Person {
//         Age:23,
//         Name: "Joe".to_owned(),
//         FavCol: String::from("Teal")
//     }
// ];
// for person in people {
//     match person.Age {
//         27 => person.print(),
//         23 => person.print(),
//         _ => continue,

//     }
// }

// impl Person {
//     fn print(&self) {
//         println!("Name is: {}",self.Name);
//         println!("Favourite colour is: {}", self.FavCol);
//     }
// }

// #[derive(Debug)]
// enum Pass {
//     Vip,
//     Backstage,
//     Standard,
// }

// #[derive(Debug)]
// struct ticket {
//     pass: Pass,
//     price: f64,
//     name: String,
// }
//     let mut people: Vec<ticket> = vec! [{ticket {pass: Pass::Vip, price: 23.6, name: String::from("Jamie")}}];
//     people.push(ticket {pass: Pass::Standard, price: 12., name: String::from("Guy")});
//     people.push(ticket {pass: Pass::Backstage, price: 19., name: String::from("Bob")});

//     for person in people {
//         match person {
//             ticket {pass: Pass::Standard,price,..}  => println!("A standard ticket at {:?}", price),
//             ticket {pass: other, price, name} => println!("A {:?} ticket at {:?} for {:?}",other,price,name),
//             _ => ()
//         }
//     }

// #[derive(Debug)]
// struct Student {
//     name: String,
//     locker: Option<i32>,
// }

// impl Student {
//     /// Creates a new [`Student`].
//     fn new(name: String, locker: Option<i32>) -> Self {
//         Self { name, locker }
//     }

//     fn print(&self) {
//         match self.locker {
//             Some(locker)  => println!("{}'s locker number is: {:?}", self.name, locker),
//             _ => println!("{} has no locker",self.name)



//         };
//     }
// }

// #[derive(Debug)]
// struct Adult {
//     name: String,
//     age: i32,
// }
// impl Adult {
//     fn new(name: &str, age: i32) -> Result<Adult,&str> {
//         match (age>21) {
//             true => Ok(Adult{name: name.to_owned(),age}),
//             _ => Err(" is not an adult."),
//         }
//     }
    
// }
    // let jamie = Adult::new("Jamie",28);
    // let young = Adult::new("young'un",13);

    // let mut people = vec![];
    // people.push(jamie);
    // people.push(young);
    // for person in people { 
    //     match person {
    //         Ok(person) => println!("{} is {} years old", person.name, person.age),
    //         Err(e) => println!("{e}"),
    //     }
    // }
    //

use std::collections::HashMap;
fn main() {
    let mut map = HashMap::new();
    map.insert("Chairs", 5);
    map.insert("Beds", 3);
    map.insert("Tables", 2);
    map.insert("Couches",0);

    for (item, stock) in map.iter() {
        match stock {
            0 => println!("{} out of stock",item),
            _ => println!("{} has {} units",item,stock),
        }
    }

}










