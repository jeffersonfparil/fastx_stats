/////////////////////
// data structures //
/////////////////////

pub struct UserInput {
    pub filename: String,
    pub flags: String
}

impl UserInput {
    pub fn new(args: &[String]) -> UserInput {
        let filename = args[1].clone();
        let flags = args[2].clone();
        UserInput { filename, flags }
    }
}

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


// Read sequence data in fasta and fastq formats
