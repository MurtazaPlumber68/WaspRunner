
fn main() {
    println!("Hello from WASM!");
    
    // Print environment variables
    for (key, value) in std::env::vars() {
        println!("{}={}", key, value);
    }
    
    // Print command line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        println!("Arguments: {:?}", &args[1..]);
    }
}
