# JUST RUST IT!
### Some Rust stuff (NodeJS is sus)

use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let mut rng = rand::thread_rng();
    const TOTAL_N: usize = 1_000_000;
    let mut i = 0;
    let mut sums: [u64; 3] = [0, 0, 0];
    let mut start = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Something went wrong 1");
    println!("Started FIRST at: {:?}", start);
    loop {
        if i == TOTAL_N {
            break;
        }

        if rng.gen_range(0..100) % 2 == 0 {
            sums[0] += 1;
        }

        i += 1;
    }
    let mut end = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Something went wrong 2");
    println!("Finished in: {:?} with sum: {}", (end - start), sums[0]);

    println!("\n----------\n");

    i = 0;
    start = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Something went wrong 1");
    println!("Started SECOND at: {:?}", start);
    while i < TOTAL_N {
        let random = rand::thread_rng().gen_range(0..100);
        if random % 2 == 0 {
            sums[1] += 1;
        }
        i += 1;
    }
    end = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Something went wrong 2");
    println!("Finished in: {:?} with sum: {}", (end - start), sums[0]);

    let arr = vec![0; TOTAL_N];
    start = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Something went wrong 1");
    println!("Started THIRD at: {:?}", start);
    for _ in arr.iter() {
        let random = rand::thread_rng().gen_range(0..100);
        if random % 2 == 0 {
            sums[2] += 1;
        }
    }
    end = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Something went wrong 2");
    println!("Finished in: {:?} with sum: {}", (end - start), sums[2]);

    let string = "Hello World";
    println!("Cannot be changed: {}", string);

    let mut mut_string = String::new();
    mut_string.push('B');
    mut_string.push_str("ello!");
    println!("Mutable string: {}", mut_string);

    let other_string = String::from("y i p p p i e e e e e");
    let mut vec: Vec<char> = other_string.chars().collect();
    vec.sort();
    vec.dedup();
    for char in vec {
        print!("{}", char);
    }
    println!();

    let str4 = "Random String";
    let str5 = str4.to_string();
    println!("Before: {}", str5);

    let str_to_bytes = str5.as_bytes();
    let str6 = &str_to_bytes[0..3];
    println!("{}", str6.len());

    let str7 = String::from("Hello");
    let str8 = String::from("World");
    let str9 = str7 + " " + &str8;
    println!("Concat: {} and {}", str9, str8);

    let int1 = 23;
    let int2 = 3;
    let int_quot = int1 / int2;
    let float_quot: f32 = int1 as f32 / int2 as f32;
    println!("Quots: {} and {}", int_quot, float_quot);

    enum Days {
        Sunday,
        Monday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
    }
    impl Days {
        fn is_weekend(&self) -> bool {
            match self {
                Days::Saturday | Days::Sunday => true,
                _ => false,
            }
        }
    }

    let today: Days = Days::Monday;
    match today {
        Days::Monday => println!("Oh nou"),
        Days::Wednesday => println!("Yippie"),
        _ => println!("A Day"),
    }
    println!("Is today the weekend? {}\n", today.is_weekend());

    // Vectors are mutable and can grow [must be of the same type]
    let vector: Vec<i32> = Vec::new();
    let mut second_vector = vec![1, 2, 3, 4];
    second_vector.push(123);
    println!("vector 1: {:?}, vector 2: {:?}", vector, second_vector);
    match second_vector.get(1) {
        Some(val) => print!("{} Val: {}", 1, val),
        None => println!("No val"),
    }
}
