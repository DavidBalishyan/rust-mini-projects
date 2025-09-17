use std::io;

// Struct to hold two numbers
pub struct Numbers {
    a: f64,
    b: f64,
}

impl Numbers {
    fn add(&self) -> f64 {
        self.a + self.b
    }

    fn subtract(&self) -> f64 {
        self.a - self.b
    }

    fn multiply(&self) -> f64 {
        self.a * self.b
    }

    fn divide(&self) -> Option<f64> {
        if self.b != 0.0 {
            Some(self.a / self.b)
        } else {
            None
        }
    }
}

pub fn main() {
    let mut input = String::new();

    println!("Enter first number:");
    io::stdin().read_line(&mut input).unwrap();
    let a: f64 = input.trim().parse().unwrap();
    input.clear();

    println!("Enter operator (+, -, *, /):");
    io::stdin().read_line(&mut input).unwrap();
    let op: char = input.trim().chars().next().unwrap();
    input.clear();

    println!("Enter second number:");
    io::stdin().read_line(&mut input).unwrap();
    let b: f64 = input.trim().parse().unwrap();

    let nums = Numbers { a, b };

    let result = match op {
        '+' => Some(nums.add()),
        '-' => Some(nums.subtract()),
        '*' => Some(nums.multiply()),
        '/' => nums.divide(),
        _ => {
            println!("Invalid operator!");
            None
        }
    };

    if let Some(r) = result {
        println!("Result: {}", r);
    }
}
