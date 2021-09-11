use std::env;

use fastx_stats::UserInput;

fn main() {
    // Splash intro message:
    println!("----------------------------------");
    println!("FASTX_STATS");
    println!("DNA sequence statistics");
    println!("----------------------------------");
    // User input:
    let args: Vec<String> = env::args().collect();
    let user_input = UserInput::new(&args);
    // // test Fasta
    // let seq1 = structs_traits_methods::Fasta {
    //     name: String::from("> chr1:123-213:blah-blah"),
    //     sequence: String::from("ATCGTAGCTATCATTAGCTAGCTAGCTGACTCATCTAGCT")
    // };
    println!("Filename: {}", user_input.filename);
    println!("Flaghs: {}", user_input.flags);
}
