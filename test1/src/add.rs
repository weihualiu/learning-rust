use std::ops::Add;

#[derive(Debug, PartialEq)]
pub struct Point2 {
    pub x: i32,
    pub y: i32,
}

impl Add for Point2 {
    type Output = Point2;

    fn add(self, rhs: Point2) -> Point2 {
        Point2{
            x : self.x + rhs.x,
            y : self.y + rhs.y,
        }
    }
}