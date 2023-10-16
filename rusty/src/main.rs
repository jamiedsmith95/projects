

fn name_print(first: &str, second: &str) {
    println!("{}", first);
    println!("{}", second);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn hgb(greet: bool)  {
    if greet {
        println!("hello");
    } else {
        println!("goodbye");
    }

}
 
fn five(num: i32) {
    if num > 5 {
        println!(">5");
    } else if num == 5 {
        println!("=5");
    } else {
        println!("<5");
    }
}
fn main() {
    let num = 5;
    five(num);
    
}
