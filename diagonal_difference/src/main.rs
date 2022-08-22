fn main() {
    println!("Diagonal Difference");

    let size = 3;
    let matrix = vec![[11, 2, 4], [4, 5, 6], [10, 8, -12]];

    println!("{size}");
    println!();

    for arr in matrix.iter() {
        for item in arr.iter() {
            print!("{}\t", item);
        }
        println!();
    }
    println!();

    // let mut primary_diagonal: i32;
    // let mut secondary_diagonal: i32;

    // for (key, <item>) in matrix.iter().enumerate().take(size) {
    for key in 0..size {
        println!("{}", matrix[key][key]);
    }
    println!();

    // for (key, <item>) in matrix.iter().enumerate().take(size) {
    for key in 0..size {
        // primary_diagonal += matrix[key][key];
        println!("{}", matrix[key][size - key - 1]);
    }
}
