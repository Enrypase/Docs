use rand::Rng;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::ops::Add;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

mod restaurant;
use crate::restaurant::order_food;

fn loops_benchmark(text: String) {
    println!("{}", text);

    let mut rng = rand::thread_rng();
    const TOTAL_N: usize = 1_000_000/*_000*/;
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
    i = 0;
    start = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Something went wrong 1");
    println!("Started SECOND at: {:?}", start);
    while i < TOTAL_N {
        let random = rng.gen_range(0..100);
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
        let random = rng.gen_range(0..100);
        if random % 2 == 0 {
            sums[2] += 1;
        }
    }
    end = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Something went wrong 2");
    println!("Finished in: {:?} with sum: {}", (end - start), sums[2]);
}

fn strings(text: String) {
    println!("{}", text);

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
}

fn casting(text: String) {
    println!("{}", text);

    let int1 = 23;
    let int2 = 3;
    let int_quot = int1 / int2;
    let float_quot: f32 = int1 as f32 / int2 as f32;
    println!("Quots: {} and {}", int_quot, float_quot);
}

fn enums(text: String) {
    println!("{}", text);

    enum Days {
        Sunday,
        Monday,
        Tuesday,
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
    let days: [Days; 7] = [
        Days::Sunday,
        Days::Monday,
        Days::Tuesday,
        Days::Wednesday,
        Days::Thursday,
        Days::Friday,
        Days::Saturday,
    ];

    println!(
        "Is today the weekend? {} {}\n",
        today.is_weekend(),
        days[0].is_weekend()
    );
}

fn vectors(text: String) {
    println!("{}", text);

    let vector: Vec<i32> = Vec::new();
    let mut second_vector = vec![1, 2, 3, 4];
    second_vector.push(123);
    println!("vector 1: {:?}, vector 2: {:?}", vector, second_vector);
    match second_vector.get(1) {
        Some(val) => print!("{} Val: {}", 1, val),
        None => println!("No val"),
    }
}

fn return_two_vals(val: i32) -> (i32, i32) {
    return (val, val * 2);
}

fn generics<T: Add<Output = T>>(x: T, y: T) -> T {
    // To perform additions with generics, you must import std::ops::Add;
    return x + y;
}

//

fn print_str(str: String) {
    println!("String {}", str);
}

fn print_return_str(str: String) -> String {
    println!("String 2: {}", str);
    return str;
}

fn change_string(str: &mut String) {
    str.push_str(" happy happy happy");
    println!("Message: {}", str);
}

fn ownership(text: String) {
    println!("{}", text);
    println!("Everytime you alloc on the heap: \n 1) You get a pointer called owner \n 2) There's only 1 owner at a time \n 3) Once the owner goes out of scopes, it disappears");
    let str1 = String::from("World");
    let str2 = str1; // At this point, str1 doesn't exist anymore
    println!("Hello {}", str2);
    let str3 = str2.clone();
    println!("Hello {}", str3);
    let str4 = String::from("Ayo ");
    print_str(str4.clone());
    let mut str5 = print_return_str(str4);
    change_string(&mut str5);
}

fn hash_maps(text: String) {
    println!("{}", text);
    let mut ips = HashMap::new();
    ips.insert("192.168.0.0", "Net Addr");
    ips.insert("192.168.0.1", "Router Addr");
    for (k, v) in ips.iter() {
        println!("{} = {}", k, v);
    }
    println!("Length: {}", ips.len());
    if ips.contains_key(&"192.168.0.0") {
        let net_addr = ips.get(&"192.168.0.0");
        match net_addr {
            Some(x) => println!("{}", x),
            None => println!("No net addr found"),
        }
    }
}

fn structs(text: String) {
    println!("{}", text);
    struct Customer {
        name: String,
        address: String,
        balance: f32,
    }
    let mut person = Customer {
        name: String::from("Enrico Pasetto"),
        address: String::from("Some Address"),
        balance: -1024.512,
    };
    person.address = String::from("New Address here");
    println!(
        "Customer: {} {} {}",
        person.name, person.address, person.balance
    );

    struct Rectangle<T, U> {
        length: T,
        height: U,
    }
    let rec = Rectangle {
        length: 10,
        height: 23,
    };
    println!("Rectangle: {} {}", rec.length, rec.height);

    trait Shape {
        fn new(length: f32, height: f32) -> Self;
        fn area(&self) -> f32;
    }
    struct Square {
        length: f32,
    }
    impl Shape for Square {
        fn new(length: f32, _: f32) -> Square {
            return Square { length };
        }
        fn area(&self) -> f32 {
            return self.length * self.length;
        }
    }
}

fn error_handling(text: String) {
    println!("{}", text);
    let path = "file.txt";
    let out = File::create(path);
    let mut out = match out {
        Ok(file) => file,
        Err(error) => panic!("Problem in file creation: {}", error),
    };
    out.write_all(b"Hello World\nLet's write something in a file")
        .expect("Failed to write to file");
    let inp = File::open(path).unwrap();
    let br = BufReader::new(inp);
    for line in br.lines() {
        println!("{}", line.unwrap());
    }

    let inp2 = File::open("random.txt");
    let inp2 = match inp2 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("random.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Could not create file not found {}", e),
            },
            _other_error => panic!("Other error {}", _other_error),
        },
    };
    for line in BufReader::new(inp2).lines() {
        print!("{}", line.unwrap());
    }
}

fn closures(text: String) {
    println!("{}", text);
    let can_vote = |age: i32| age >= 18;
    println!("Can vote? {}", can_vote(32));

    let mut samp1 = 5;
    let print_var = || println!("Samp1 = {}", samp1);
    print_var();
    samp1 = 10;
    let mut change_var = || samp1 += 1;
    change_var();
    println!("Incr samp1 = {}", samp1);

    fn use_func<T>(a: i32, b: i32, func: T) -> i32
    where
        T: Fn(i32, i32) -> i32,
    {
        func(a, b)
    }
    let sum = |a, b| a + b;
    let prod = |a, b| a * b;
    println!("5 + 4 = {}", use_func(5, 4, sum));
    println!("5 + 4 = {}", use_func(5, 4, prod));
}

fn smart_pointers(text: String) {
    println!("{}", text);
    // Box => Large amount of data on heap, you pass a pointer to it in the stack
    /*
        Cannot create it like this:
        struct TreeNode<T> {
            pub left: TreeNode<T>,
            pub right: TreeNode<T>,
            pub key: T,
        }
    */
    struct TreeNode<T> {
        pub left: Option<Box<TreeNode<T>>>,
        pub right: Option<Box<TreeNode<T>>>,
        pub key: T,
    }
    impl<T> TreeNode<T> {
        pub fn new(key: T) -> Self {
            TreeNode {
                left: None,
                right: None,
                key,
            }
        }
        pub fn left(mut self, node: TreeNode<T>) -> Self {
            self.left = Some(Box::new(node));
            self
        }
        pub fn right(mut self, node: TreeNode<T>) -> Self {
            self.right = Some(Box::new(node));
            self
        }
    }

    let node1 = TreeNode::new(1);
    println!("Node key: {}", node1.key);
    node1.left(TreeNode::new(2)).right(TreeNode::new(3));
}

fn concurrency(text: String) {
    println!("{}", text);
    let thread1 = thread::spawn(|| {
        for i in 1..25 {
            println!("Spawned thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..20 {
        println!("Main thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    thread1.join().unwrap();

    pub struct Bank {
        balance: f32,
    }
    fn withdraw(bank: &Arc<Mutex<Bank>>, amount: f32) {
        let mut bank_ref = bank.lock().unwrap();
        if bank_ref.balance < 5.0 {
            println!(
                "Current balance: {} Withdraw a smaller amount!",
                bank_ref.balance
            );
        } else {
            bank_ref.balance -= amount;
            println!(
                "Customer withdrew {}, balance: {}",
                amount, bank_ref.balance
            );
        }
    }
    fn customer(bank: Arc<Mutex<Bank>>) {
        withdraw(&bank, 5.0);
    }
    let bank: Arc<Mutex<Bank>> = Arc::new(Mutex::new(Bank { balance: 1000.0 }));
    let handles = (0..10).map(|_| {
        let bank_ref = bank.clone();
        thread::spawn(|| {
            customer(bank_ref);
        })
    });
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Total {}", bank.lock().unwrap().balance);
}

fn main() {
    loops_benchmark(String::from(
        "\n ====== RUNNING A LITTLE LOOPS BENCHMARK ====== \n",
    ));
    strings(String::from("\n ====== STRINGS TUTORIAL ====== \n"));
    casting(String::from("\n ====== CASTING TUTORIAL ====== \n"));
    enums(String::from("\n ====== ENUMS TUTORIAL ====== \n"));
    vectors(String::from("\n ====== VECTORS TUTORIAL ====== \n"));
    println!("\n ====== FUNCTIONS TUTORIAL ====== \n");
    let (val1, val2) = return_two_vals(32);
    println!("Got {} and {}", val1, val2);
    println!("\n ====== GENERICS TUTORIAL ====== \n");
    println!("5 + 4 = {}", generics(5, 4));
    ownership(String::from("\n ====== OWNERSHIP TUTORIAL ====== \n"));
    hash_maps(String::from("\n ====== HASH MAP TUTORIAL ====== \n"));
    structs(String::from("\n ====== STRUCTS TUTORIAL ====== \n"));
    println!("\n ====== MODULES TUTORIAL ====== \n");
    order_food();
    error_handling(String::from("\n ====== MODULES TUTORIAL ====== \n"));
    closures(String::from("\n ====== CLOSURES TUTORIAL ====== \n"));
    smart_pointers(String::from("\n ====== CLOSURES TUTORIAL ====== \n"));
    concurrency(String::from("\n ====== CLOSURES TUTORIAL ====== \n"));
}
