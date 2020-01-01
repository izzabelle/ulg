fn main() {
    // match the result for error handling rn
    match ulg_rs::main_wrapper() {
        Ok(_) => std::process::exit(0),
        Err(e) => {
            eprintln!("Ulg encountered an error: {:?}", e);
            std::process::exit(1);
        }
    }
}
