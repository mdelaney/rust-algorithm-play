mod strings;

fn main() {
    println!("Hello, world!");
    let a: String = String::from("foo");
    let b: String = String::from("booba");
    let result = strings::edit_distance::levenshtein_distance(&a, &b);
    println!("result is: {}", result);
}
