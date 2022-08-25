fn main() {
    let elements = [1, 2, 3, 4, 10, 11];

    // Cach 1
    // let mut result = 0;
    // for element in elements {
    //     result += element;
    // }

    // Cach 2
    let result: i32 = elements.iter().sum();

    println!("Sum: {}", result);
}
