use std::{io, thread};
use std::io::Read;
use std::ops::{Add, Deref};
use std::collections::HashMap;
use std::fs::File;
use std::io::ErrorKind;
use test1::{Tweet, NewArticle, Summary};
use crate::List::{Cons, Nil};
use std::thread::JoinHandle;
use std::time::Duration;
use std::sync::mpsc;
use test1::add::Point2;

const PATH_SIZE :u32 = 256;

fn main() {
    {
        println!("Hello, world!");
        println!("Please input your guess.");
        let mut guess = String::new();

        // io::stdin().read_line(&mut guess).expect("Failed to read line");
        // println!("Your guessed: {}", guess);

        let name : u32 = 15;
        println!("nihao {}", name.to_string());
        let name  = "test....";
        println!("{}", name);
        println!("{}", name);

        let mut x = 5;
        println!("{}", x);
        x = 6;
        println!("{}", x);
        let x = x * 2;
        println!("{}", x);

        let flag:bool = true;
        let c = 'z'; // char 4bytes
        // 复合类型  元组tuple和数组array
        let tup:(i32, f64, u8) = (500, 1.0, 1);
        let (x,y,z) = tup;
        let a = [1,2,4,5,6]; // 一旦声明长度固定
        let b = a[3];
        // let c = a[x as usize];
        // 强制转化，使用as
        println!("sum(21,51) = {}", sum(x,51));

        let x = if sum(x, 51) < x {
            true
        }else { false };
        println!("{}", x);

        let x = 5;
        let y = x;
        println!("{}", x);

        //
        let x = String::from("hello world");
        let (y,z) = test_string(x);
        let mut y = y;
        ref_string(&y);
        ref_mut_string(&mut y);
        println!("{}, {}", y, z);
    }

    {
        let mut user1 = User{
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active:true,
            sign_in_count:1
        };
        println!("user = {:?}", user1);
        user1.sign_in_count = 3;

        let username = user1.getUserName();
        println!("name = {}", username);
        user1.setUserName("beijing");
        println!("name = {}", user1.username);

        let user2 = User {
            username : String::from("shanghai"),
            sign_in_count : 2,
            email : String::from("a@a.com"),
            active: false
        };

        let can_hold_result = user1.can_hold(&user2);
        println!("can_hold result = {}", can_hold_result);

        let user3 = User::instance("tianjin", "tianjin@cctv.com");
        println!("user3 = {:?}", user3);
    }

    // enum
    {
        let first = IPAddrKind::V4;
        let second = IPAddrKind::V6;
        let loopback = IPAddr{
            kind:IPAddrKind::V6,
            address : String::from("::"),
        };
        let home = IPAddr{
            kind : IPAddrKind::V4,
            address : String::from("127.0.0.1"),
        };
        let home = IpAddr::V4(String::from("127.0.0.1"));
        let loopback = IpAddr::V6(String::from("::"));

        println!("ip addr: {}", getIpAddr(home));
        println!("ip addr2: {}", getIpAddr(loopback));

    }

    // vector
    {
        let mut v = Vec::new();
        v.push(5);
        v.push(6);
        v.push(7);

        let v = vec![1,2,3,4,5];
        let third = &v[1];
        println!("the third element is {}", third);
        println!("the last element is {}", v.get(v.len() -1 ).unwrap());

        let mut v = Vec::new();
        v.push(3);
        v.push(4);
        v.push(5);
        let va = &v[0];
        let vb = &v[1];
        // v.push(7);
        println!("the first element is: {}", va);

        for i in &mut v {
            println!("{}", i);
        }

        let mut cores = HashMap::new();
        cores.insert(String::from("Blue"), 10);
        cores.insert(String::from("Yellow"), 50);
        for (key, value) in &cores {
            println!("key:{} value:{}", key, value);
        }
    }

    // panic!("crashdump!!!");
    // error
    {
        let f = File::open("/Users/liuweihua/Downloads/aaa.dat");
        let f = match f {
            Ok(f1) => f1,
            Err(err) =>
                match err.kind() {
                    ErrorKind::NotFound => match File::create("/Users/liuweihua/Downloads/aaa.dat") {
                        Ok(fc) => fc,
                        Err(e) => panic!("Problem creating the file: {:?}", e),
                    },
                    other_error => panic!("Problem creating the file: "),
                },
        };
    }

    // trait
    {
        let char_list = vec!['y', 'm', 'a', 'q'];
        let res = largest(&char_list);
        println!("result: {}", res);

        let integer = Point{x : 12, y :24};
        let float = Point{x: 2.3, y :4.5};
        println!("x: {}", integer.x());
        println!("float {}", float.distance_from_origin());

        // 定义共享行为
        let trait1 = NewArticle{
            headline: String::from("c plus plus"),
            location: String::from("beijing"),
            author: String::from("fuck c/c++"),
            content: String::from("from c/c++ to Rust"),
        };
        println!("{}, {}", trait1.summarize() ,trait1.getString());
        let trait2 = Tweet{
            username: String::from("Tweet"),
            content: String::from("content"),
            reply: false,
            retweet: false
        };
        println!("{}, {}", trait2.summarize(), trait2.getString());
    }

    // smart point
    {
        let b = Box::new(5);
        println!("b = {}", b);
        let list = Cons(1,
                        Box::new(Cons(2,
                                      Box::new(Cons(3,
                                                    Box::new(Cons(4,
                                                                  Box::from(Nil))))))));

        let x = 5;
        let y = Box::new(x);
        assert_eq!(5, x);
        assert_eq!(5, *y);

        let x = 5;
        let y = MyBox::new(x);
        assert_eq!(5, x);
        assert_eq!(5, *y);

        let x = MyBox::new(String::from("hello world"));
        hello(&**x);
    }

    // thread
    {
        let handler = thread::spawn(||{
            for i in 1..10 {
                println!("hi number {} from the spawned thread", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {} from the main thread", i);
            thread::sleep(Duration::from_millis(1));
        }

        handler.join().unwrap();

        let v = vec![1,2,3];
        let handle = thread::spawn(move || {
            println!("Here is a vector: {:?}", v);
        });
        // println!("{:?}", v);
        handle.join().unwrap();

    }

    // message transaction
    {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
            // println!("{}", val);
        });

        let received = rx.recv().unwrap();
        println!("Got: {}", received);
    }

    // unsafe
    {
        unsafe {
            dangous();
        }

        add_to_count(23);
        unsafe {
            println!("unsafe COUNTER: {}", COUNTER);
        }

    }

    // advandce trait
    {
        assert_eq!(Point2{x:1,y:2} + Point2{x:2,y:3}, Point2{x:3, y: 5});
    }
}

fn sum(x : i32, y : i32) -> i32 {
    if x > 5 {
        return x + 5 + y;
    }else if y > 5 {
        return x - 5 + y;
    }
    x + y
}

fn test_string(str : String) -> (String, String) {
    (str, String::from("test2"))
}

fn ref_string(str : &String) {
    println!("ref String = {}", str);
}

fn ref_mut_string(str :&mut String) {
    str.push_str(" nihao");
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    fn getUserName(&self) -> &str {
        self.username.as_str()
    }

    fn setUserName(&mut self, name : &str) {
        self.username = String::from(name);
    }

    fn can_hold(&self, other : &User) -> bool {
        self.sign_in_count > other.sign_in_count
    }

    fn instance(username: &str, email: &str) -> User {
        User{
            username :String::from(username),
            email:String::from(email),
            sign_in_count : 0,
            active : false,
        }
    }
}

enum IPAddrKind {
    V4,
    V6,
}

struct IPAddr {
    kind : IPAddrKind,
    address: String,
}

enum IpAddr {
    V4(String),
    V6(String),
}

fn getIpAddr(ipaddr : IpAddr) -> String {
    match ipaddr {
        IpAddr::V4(IpV4) => IpV4.clone(),
        IpAddr::V6(IpV6) => IpV6.clone(),
    }
}

fn largest<T : std::cmp::PartialOrd>(list : &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

enum Languge<Syntax, Code> {
    Ok(Syntax),
    Err(Code),
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T> (T);

impl<T> MyBox<T>{
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name : &str) {
    println!("Hello, {}!", name);
}

unsafe fn dangous() {

}

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe{
        COUNTER+=inc;
    }

}
