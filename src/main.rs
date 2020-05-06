use std::io;
const NUM_OF_ENTRIES: usize = 5;


fn main() {
    // Define the length of array using an integer
    let length: f32 = 5.0;
    //Initialize arrays to hold the 5 numbers and enter the arrays
    let mut _a: [f32; NUM_OF_ENTRIES] = create_arrays();
    let mut _b: [f32; NUM_OF_ENTRIES] = create_arrays();
    // Determine total number of number of entries
    let total_entries: usize = length*2;
    // Determine the sum of the array_sum
    let sum_a: f32 = sum(_a);
    let sum_b: f32 = sum(_b);
    // Determine the grand total
    let grand_total: f32 = sum_a + sum_b;
    println!("Grand total: {}", grand_total);
    // Correction factor
    let correction_factor: f32 = grand_total*grand_total/total_entries;
    println!("Correction factor: {}", correction_factor);
}

fn create_arrays() -> [f32; NUM_OF_ENTRIES] {
    println!("Please input first list of 5 numbers");
    let mut array_1 = [0.0; NUM_OF_ENTRIES];
    for i in 0..NUM_OF_ENTRIES {
        // Initialize the variable to hold the number
        let mut entry = String::new();
        io::stdin().read_line(&mut entry)
            .expect("Failed to read number");
        let entry: f32 = match entry.trim().parse() {
            Ok(num) => num,
            Err(_) => 0.1,
        };
        array_1[i] = entry;
    }
    println!("The array you entered is: {:?}", array_1);
    return array_1;
}

fn sum(array: [f32;NUM_OF_ENTRIES]) -> f32 {
    let mut array_sum: f32 = 0.0;
    for i in 0..NUM_OF_ENTRIES {
        array_sum = array_sum + array[i];
    }
    return array_sum;
}
