use std::io;

fn main() {
    println!("Hello, world!");

    let n: u32 = loop {
        println!("Enter n to get the n-th Fibonacci number.");
        
        let mut n = String::new();
        
        io::stdin()
            .read_line(&mut n)
            .expect("Reading n failed!");

        match n.trim().parse() {
            Err(_) => {},
            Ok(num) => break num
        }
    };

    println!("\nInteger computation:");

    if n == 0 {
        println!("{}", 0);
        return
    } else if n == 1 {
        println!("{}", 1);
        return
    }

    let mut a: u128 = 0;
    let mut b: u128 = 1;

    let mut overflow = false;

    for _ in 2..=n {
        let sum = a.overflowing_add(b);

        a = b;
        b = sum.0;
        overflow = sum.1;

        if overflow {
            break
        }
    }

    if overflow {
        println!("Numbers too big!");
    } else {
        println!("{}", b);
    }

    println!("\nFloating point computation:");

    if n == 0 {
        println!("{}", 0.0);
        return
    } else if n == 1 {
        println!("{}", 1.0);
        return
    }

    let mut a: f64 = 0.0;
    let mut b: f64 = 1.0;

    for _ in 2..=n {
        let c = a + b;
        a = b;
        b = c;
    }

    println!("{}", b);
}
