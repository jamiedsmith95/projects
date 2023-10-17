

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
//         println!("{iter:?}");
//         if (iter == end) { break;}
//         iter = iter + 1;

//     }
// }

// fn countdown(mut from: i32)  {
//     // put mut before function arg to indicate its mutable inside, num put in doesn't need to be
//     // mut
//     while (from > 0) {
//         println!("{from:?}");
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
//     println!("The quantity is: {:?}", my_item.quantity );

// }
// fn disp_id(my_item: &item) {
//     println!( "The id is: {:?}", my_item.id);
// }

// enum Colour {
//     Blue,
//     Green,
//     Red
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
//         println!("Box has dimensions of {:?}, {:?}, {:?}", self.dimension.x,self.dimension.y,self.dimension.z);
//         println!("Box weighs {:?}",self.weight);
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

fn main() {
    let nums = vec![10,20,30,40];
    for num in nums {
        if (num ==30) {
            println!("thirty");
        } else {
            println!("{:?}",num);
        }
    }








}





