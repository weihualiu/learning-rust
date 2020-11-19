pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {

}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}