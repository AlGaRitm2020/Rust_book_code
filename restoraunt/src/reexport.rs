pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use front_of_house::hosting; // reexport. It's mean that you can use hosting even if you only import current file. In other way it raise error

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();


}
