fn main() {
    //constants
    const MAX_POINTS: u32 = 100_000;

    //mutablility
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    //shadowing
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();  //This saves us from having to create extra variables like spaces_str.

    //arithmatic
    let sum = 40 + 2;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    //booleans
    let t = true;
    let f: bool = false;

    //characters
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    //tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;    //pattern matching!
    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0; //direct acces
    let six_point_four = x.1;
    let one = x.2;

    //arrays
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = [3; 5]; // b = [3, 3, 3, 3, 3];
    let first = a[0];

    another_function(5,6);
    controlFlow();
}

//functions
//lowercase + underscored
//must declare the type of each parameter
fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn controlFlow() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    //loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    //for loops
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
       println!("{}!", number);
   }
   println!("LIFTOFF!!!");
}
