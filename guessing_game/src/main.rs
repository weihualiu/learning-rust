//use std::io;

use std::collections::HashMap;
use std::ops::Deref;
use std::thread;
use std::time::Duration;

fn main() {
    /*
    println!("Guess the number!");
    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("You guessed: {}", guess);
    */
    let momo_age = 1.5;
    println!("momo：{}", momo_age);

    let str_a = String::from("hello");
    let str_b = str_a.clone(); // move ptr address
    println!("{}, {}", str_a, str_b);
    // 如果一个类型拥有 Copy trait，一个旧的变量在将其赋值给其他变量后仍然可用
    // Rust 不允许自身或其任何部分实现了 Drop trait 的类型使用 Copy trait。
    // 类型是Copy的：整数类型、布尔类型、浮点数类型、字符类型、元组

    //takes_ownership(str_b);
    ref_string(&str_b);
    println!("{}", str_b);
    println!("{}", first_word(&str_b));

    let int_a: i32 = 5;
    make_copy(int_a);
    println!("{}", int_a);

    let user = User {
        username: String::from("liuweihua"),
        email: String::from("auhiewuil@gmail.com"),
        active: true,
        sign_in_count: 10,
    };
    println!(
        "{}, {:?}, {}",
        user.get_username(),
        user,
        user.compare_sing_in_count(&User {
            username: String::from("test"),
            email: String::from("test@test.com"),
            active: false,
            sign_in_count: 15,
        })
    );
    //println!("{}", build_user(String::from("admin@admin.com"), str_b).username);
    // println!("{}", str_b);

    let mut v: Vec<i32> = Vec::new();
    //let v = vec![1,2,3];
    v.push(5);
    v.push(6);
    v.push(7);
    println!("vector: {:?}", v);

    {
        let mut vec_str: Vec<&str> = Vec::new();
        vec_str.push(&str_a);
        vec_str.push(&str_b);
        println!("vector str: {:?}", vec_str);

        match vec_str.get(2) {
            Some(third) => println!("The third element is {}", third),
            None => println!("There is no third element"),
        }
    }
    println!("{}, {}", str_a, str_b);
    let first = v[0];
    v.push(8);
    println!("The first element is : {}", first);
    for i in &mut v {
        *i += 50;
    }
    println!("for +50 : {:?}", v);

    let mut s = String::from("foo");
    s.push_str("bar");
    let s2 = "bar";
    s.push_str(s2);
    println!("s: {}, {}", s, s2);
    println!("s2: {}", s2);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("s3: {}. s2: {}", s3, s2);
    let s1 = "test";
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s: {}", s);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("scores: {:?}", scores);

    //let teams = vec![String::from("Blue"), String::from("Yellow")];
    //let initial_scores = vec![10,50];
    //let scores : HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect();
    //println!("scores: {:?}", scores);
    let team_name = String::from("Blue");
    println!("bule score: {}", scores.get(&team_name).unwrap());
    scores.insert(String::from("Blue"), 20);
    scores.entry(String::from("Red")).or_insert(30);
    scores.entry(String::from("Yellow")).or_insert(40);
    for (key, value) in &scores {
        println!("{} {}", key, value);
    }

    // generic
    let integer = Point{x:5, y:10};
    let float = Point{x:1.0, y:4.0};
    let complex = Point{x:1, y:4.0};
    println!("{:?}, {:?}, {:?}", integer, float, complex);
    println!("point func: {}", float.distance_from_origin());
    let point_char = Point{x:"Hello",y:'c'};
    //println!("{:?}", integer.mixup(point_char));
    println!("{}", integer.get_class());
    println!("{}", get_student_class(integer));
    let vec_i32 = vec![1,54,13,53,6,32];
    println!("{}", largest(&vec_i32));

    {
        let r;
        //{
            let x = 5;
            r = &x;
        //}
        println!("r: {}", r);
    }

    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    let b = Box::new(5);
    println!("b = {}", b);    

    let node1 = Node{next: Link::Empty , value: String::from("node1")};
    let node2 = Node{next: Link::More(Box::new(node1)), value : String::from("node2")};
    let node3 = Node{next: Link::More(Box::new(node2)), value: String::from("node3")};
    println!("node is {:?}", node3);

    let x = 5;
    let y = MyBox::new(x);

    println!("y {}", *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();

}


fn largest<T: PartialOrd + Copy>(list : &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    }else{
        y
    }
}

fn _takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn make_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn ref_string(s: &String) -> usize {
    s.len()
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}

impl User {
    fn get_username(&self) -> &String {
        &self.username
    }

    fn compare_sing_in_count(&self, other: &User) -> bool {
        if self.sign_in_count > other.sign_in_count {
            true
        } else {
            false
        }
    }
}

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    } 
}

impl<T,U> Point<T,U> {
    fn mixup<V,W>(self, other: Point<V,W>) -> Point<T,W> {
        Point{
            x: self.x,
            y: other.y,
        }
    }
}

pub trait Student {
    fn get_class(&self) -> u32;
}

impl<T> Student for Point<u32, T> {
    fn get_class(&self) -> u32 {
        self.x
    }
}

fn get_student_class(s: impl Student) -> u32 {
    s.get_class()
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug)]
struct Node {
    next : Link,
    value : String,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}", name);
}