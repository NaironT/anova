use std::io;
const NUM_OF_ENTRIES: usize = 5;


fn main() {
    println!("Please input first list of 5 numbers");

    //Initialize arrays to hold the 5 numbers
    let mut _a: [f32;NUM_OF_ENTRIES] = create_arrays();
    let mut _b: [f32;NUM_OF_ENTRIES] = create_arrays();
}

fn create_arrays() -> [f32;NUM_OF_ENTRIES] {
    let mut array_1 = [0.0; NUM_OF_ENTRIES];
    for i in 0..NUM_OF_ENTRIES {
        // Initialize the variable to hold the number
        let mut entry = String::new();
        io::stdin().read_line(&mut entry)
            .expect("Failed to read number");
        let entry: f32 = entry.trim().parse()
            .expect("Please input a number");
        array_1[i] = entry;
    }
    println!("The array you entered is: {:?}", array_1);
    return array_1;
}
