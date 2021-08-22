// data structures

pub struct Fasta {
    pub name: String,
    pub sequence: String
}

pub struct Fastq {
    pub name: String,
    pub quality: String,
    pub sequence: String
}

pub trait ParseSequence {
    fn parse_name(&self) -> String;
    fn parse_sequence(&self) -> String;
    fn parse_quality(&self) -> f64;
}