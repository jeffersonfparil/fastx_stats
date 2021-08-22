mod structs_traits_methods;

fn main() {
    println!("FASTX_STATS");
    println!("DNA sequence statistics");
    // test Fasta
    let seq1 = structs_traits_methods::Fasta {
        name: String::from("> chr1:123-213:blah-blah"),
        sequence: String::from("ATCGTAGCTATCATTAGCTAGCTAGCTGACTCATCTAGCT")
    };
    println!("Sequence: {}", seq1.name);
    println!("{}", seq1.sequence);
}
