/* --- BEGIN Variable Definitions ---
Function f();
Owner x;
StaticRef s;
Function String::from();
Function println!();
--- END Variable Definitions --- */
fn main() {
    let x = String::from("ABC"); // !{ Move(String::from()->x) }
    for i in 1..10 { // !{ StartLoop() }
        f(&x) // !{ PassByStaticReference(x->f()) }
    } // !{ EndLoop() }
} // !{ GoOutOfScope(x) }

fn f(s : &String) { // !{ InitRefParam(s) }
    println!("{}", *s); // !{ PassByStaticReference(s->println!()) }
} // !{ GoOutOfScope(s) }