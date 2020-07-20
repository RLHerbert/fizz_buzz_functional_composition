fn main() {
    // fizz_buzz();

    fizz_buzz_functional();
}

fn fizz_buzz() {
    // Plays the Fizz Buzz game without extraneous work.
    for num in 1..100 {
        match num % 3 == 0 {
            true => match num % 5 == 0 {
                true => println!("fizz buzz"),
                false => println!("fizz"),
            },
            false => match num % 5 == 0 {
                true => println!("buzz"),
                false => println!("{}", num),
            },
        }

        // Alternatively, with one println!, but not really any cleaner.
        // println!(
        //     "{}",
        //     match num % 3 == 0 {
        //         true => match num % 5 == 0 {
        //             true => String::from("fizz buzz"),
        //             false => String::from("fizz"),
        //         },
        //         false => match num % 5 == 0 {
        //             true => String::from("buzz"),
        //             false => format!("{}", num),
        //         },
        //     }
        // )
    }
}

fn fizz_buzz_functional() {
    for num in 1..=100 {
        // Should probably move this out of the loop.
        let discriminator = Box::new(
            |divisor: i32, // The divisor to check against for "Fizz" and "Buzz", 3 and 5 respectively.
             string: &str, // "Fizz" or "Buzz"
             function: Box<dyn Fn(&str) -> String>|
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

        println!("{}", fizz(buzz(identity))(&num.to_string()[..])); // IT WOOOOOOOOOOORKS

        // The following also works, but reuses code >:( :
        // let fizz = |func: Box<dyn Fn(&str) -> String>| {
        //     match num % 3 == 0 {
        //         true =>  Box::new(move |_s: &str| format!("{}{}","Fizz" , func(""))),
        //         false => func,
        //     }
        // };
        // let buzz = |func: Box<dyn Fn(&str) -> String>| {
        //     match num % 5 == 0 {
        //         true =>  Box::new(move |_s: &str| format!("{}{}","Buzz" , func(""))),
        //         false => func,
        //     }
        // };

        // let id = Box::new(|s: &str| s.to_string());
        // println!("{}", fizz(buzz(id))(&num.to_string()[..]));
    }
}
