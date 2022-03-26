fn largest(list: &[i32]) -> i32 {

    let mut largest = list[0];

    for &number in list {
        if number > largest {
            largest = number;
        }
    } 

    largest
}
fn main() {

    let number_list = vec![34, 50, 25, 100, 65];
    
    let result = largest(&number_list);

    println!("The largest number is {}", result);

    let number_list_2 = vec![102, 32, 6000, 98, 52, 1, 73, 8];

    let result_2 = largest(&number_list_2);
    println!("The largest number is {}", result_2);

}

