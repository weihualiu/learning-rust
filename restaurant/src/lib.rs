mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    mod serving {
        fn take_order() {}
        fn server_order() {}
        fn take_payment() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    println!("I'd like {} toast please", meal.toast);
}

mod back_of_house {
    pub struct Breakfast {
        pub toast :String,
        seasonal_fruit :String,
    }

    impl Breakfast {
        pub fn summer(toast :&str) -> Breakfast {
            Breakfast {
                toast : String::from(toast),
                seasonal_fruit : String::from("peaches"),
            }
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width < other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn front_of_house() {
        crate::eat_at_restaurant();
        assert_eq!(true, true);
    }

    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle{width: 8, height: 7};
        let smaller = Rectangle{width: 5, height: 1};
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn larger_cannot_hold_smaller() {
        let larger = Rectangle{width: 8, height: 7};
        let smaller = Rectangle{width:5, height: 1};
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_add_two() {
        assert_eq!(4, add_two(2));
    }
}
