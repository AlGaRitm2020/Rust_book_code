struct Point<T> {
    x: T,
    y: T,
}

// implementation only for float type
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}


// implementation for other types
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}



fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());

    let p2 = Point { x: 3.2, y: 2.3 };
    println!("Distance from origin is {}", p2.distance_from_origin());
}

