mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting::add_to_waitlist;


pub fn eat_at_restaraunt() {
    add_to_waitlist();
    add_to_waitlist();
    add_to_waitlist();

}

