fn main() {
    let x = String::from("ABC");
    for i in 1..10 {
        f(&x)
    }
}

fn f(s : &String) {
    println!("{}", *s);
}