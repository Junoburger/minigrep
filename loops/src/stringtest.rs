fn main() {
    // let s1 = String::from("hello");
    // let s2 = s1.clone();

    // println!("{}, world! {}", s1, s2);

    let mut s = String::from("hello"); // s comes into scope
    {
        let tst1 = &mut s;
        println!("{} test in scope", tst1);
    } // tst1 goes out of scope here, so we can make a new reference to it

    let tst2 = &mut s;

    // takes_ownership(s); // s's value moves into the function...
    // println!("{}", s); // this throws an error

    let mut s1 = String::from("hello0000");

    let len = calculate_length(&mut s1);

    println!("The length of '{}' is {}. test {}", s1, len, tst2);
}

// fn takes_ownership(some_string: String) {
//     // some_string comes into scope
//     println!("{} from function", some_string);
// } // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed

fn calculate_length(s: &mut String) -> usize {
    s.len() // len() returns the length of a String
}
