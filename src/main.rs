fn concatenate_strings(str1: &str, str2: &str) -> String {
    let mut result = String::new();
    result.push_str(str1);
    result.push_str(str2);
    result
}
 
fn main() {
    // Step 2: Initialize two String variables
    let string1 = String::from("Hello, ");
    let string2 = String::from("Rust!");

    // Step 3: Call the concatenate_strings function with string slices
    let concatenated_string = concatenate_strings(&string1, &string2);

    // Step 4: Print the result to the console
    println!("{}", concatenated_string);

    // Step 5: Compile and run the program to test its functionality
}
