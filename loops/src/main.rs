fn main() {
    // let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // let mut index = 0;

    // while index < a.len() {
    //     println!("{}", a[index]);
    //     index += 1;
    // }
    fn test<'test>(x: &'test String) {
        println!("{}", x);
    }

    for element in 10..101 {
        println!("{}", element);
        test(&"test".to_owned());
    }

    // let mut count = 0;
    // 'counting_up: loop {
    //     println!("count = {}", count);
    //     let mut remaining = 10;

    //     loop {
    //         println!("remaining = {}", remaining);
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }

    //     count += 1;
    // }
    // println!("End count = {}", count);
}
