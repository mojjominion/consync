pub fn print_usage(program_name: &str) {
    println!("Usage: {} [options] <input>", program_name);
    println!();
    println!("Options:");
    println!("    help     Print this help message");
    println!("    find     Print the version information");
    // Add more options here
    println!("Input:");
    println!("    [<input>]  File extension, e.g., config, yaml");
}
