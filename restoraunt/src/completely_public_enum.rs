fn serve_order() {}

mod back_of_house {
    pub enum Appetizer {
        Soup, 
        Salad,

    }
}


pub fn eat_at_restaraunt() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

}

