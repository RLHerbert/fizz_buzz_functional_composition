// A solution to the Fizz Buzz problem without extraneous work or duplicate checks, written in Rust.
// See README.md for additional information.

fn main() {
    for num in 1..101 {
        println!("{}", fizz_buzz_by_composition(num));
    }
}

fn fizz_buzz_by_composition(num: i32) -> String {
    let discriminator = Box::new(
        |divisor: i32, // The divisor to check against for "Fizz" and "Buzz", 3 and 5 respectively.
         string: &str, // "Fizz" or "Buzz."
         function: Box<dyn Fn(&str) -> String>| // The function which we compose.
         -> Box<dyn Fn(&str) -> String> {
            let string = string.to_string();
            match num % divisor == 0 {
                true => Box::new(move |_s: &str| format!("{}{}", string, function(""))),
                false => function,
            }
        },
    );

    let identity = Box::new(|s: &str| s.to_string());
    let buzz = |closure| discriminator(5, "Buzz", closure); // closure is the closure (function) to evaluate or return in discriminator
    let fizz = |closure| discriminator(3, "Fizz", closure);

    fizz(buzz(identity))(&num.to_string()[..])
}
