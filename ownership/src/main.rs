fn main() {

    let s = "hello";
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!`

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);


    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    // These ampersands are references, and they allow you to refer to some value without taking ownership of it. Figure 4-5 shows a diagram.
    println!("The length of '{}' is {}.", s1, len);

    let s = String::from("hello world");

    //slices
    let hello = &s[0..5];   //this becomes a reference to part of the string.
    let world = &s[6..11];

    let s = String::from("hello");
    let len = s.len();

    let slice = &s[0..len];
    let slice = &s[..]; //leaving out the start assumes 0. leaving out the end assumes len.

    let s3 = String::from("Apples Oranges Bananas");
    let first_word = first_word_prime(&s3);
    println!("The first word of '{}' is {}.", s3, first_word);

    //array slice
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

//This method is impractical since the value it returns may be invalid at some point. See: Slices.
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_prime(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
