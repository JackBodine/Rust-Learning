fn main(){

    let x = fahrenheit_to_celsius(31.0);
    println!("31 degrees fahrenheit is {} degrees celsius", x);

    let x = fibonacci(11);
    println!("{} is the 10th fibonacci number", x);

    twelve_days();

}

fn fahrenheit_to_celsius(x:f32) -> f32{
    (x - 32.0) * (5.0/9.0)
}

fn fibonacci(x:u32) -> u32{
    if x == 0 {
        return 0;
    } else if x == 1 {
        return 1;
    } else {
        return fibonacci(x-1) + fibonacci(x-2);
    }
}

fn twelve_days(){
    let mut count = 0;
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eight", "nineth", "tenth", "eleventh", "twelfth"];
    let gifts = ["and a partridge in a pear tree.","Two turtle doves,","Three French hens,","four calling birds,","five gold rings,","six geese a-laying,","seven swans a-swimming","eight maids a-milking","nine ladies dancing","ten lords a-leaping","eleven pipers piping","twelve drummers drumming"];

    while count < 12{
        println!("\nOn the {} day of Christmas my true love sent to me...", days[count]);

        let mut gift_counter = count;
        while gift_counter >= 0 {

            if gift_counter != 0 {
                println!("{}",gifts[gift_counter]);
                gift_counter = gift_counter - 1;
            }else if count == 0{
                println!("A partridge in a pear tree.");
                break
            }else{
                println!("{}",gifts[0]);
                break
            }

        }

        count = count + 1;
    }
}
