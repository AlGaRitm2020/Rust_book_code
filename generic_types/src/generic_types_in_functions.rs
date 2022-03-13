// two functoins for different types
fn largest_i32(list: &[i32]) -> i32 {

    let mut largest = list[0];

    for &number in list {
        if number > largest {
            largest = number;
        }
    } 

    largest
}

fn largest_char(list: &[char]) -> char {

    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    } 

    largest
}

// one function for any type
fn largest<T>(list: &[T]) -> T{

    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    } 

    largest
}

fn main() {

    let number_list = vec![34, 50, 25, 100, 65];
    
    let result = largest_i32(&number_list);
    let result_3 = largest(&number_list);

    println!("The largest number is {}, {}", result, result_3);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result_2 = largest_char(&char_list);
    let result_4 = largest(&char_list);
    println!("The largest char is {}, {}", result_2, result_4);

}

