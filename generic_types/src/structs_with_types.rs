struct Point<T> {
    x: T,
    y: T,
}

struct Point_2<T, U> {
    x: T,
    y: U,
}


fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let wont_work = Point { x: 5, y: 4.0 }; // compiler think that T is integer and raise an error when get 4.0
    let integer_and_float = Point_2 { x: 5, y: 4.0 }; // it's ok because Point_2 have 2 generic types
    
}

