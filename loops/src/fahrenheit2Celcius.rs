fn main() {
    let mut fahrenheit = 0;
    while fahrenheit <= 100 {
        let celcius = (fahrenheit - 32) * 5 / 9;
        println!("{}°F is {}°C", fahrenheit, celcius);
        fahrenheit += 10;
    }
}
