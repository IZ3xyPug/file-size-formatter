use std::str::FromStr;
use std::env;
#[derive(Debug)]
enum FileSize {
    Bytes,
    Kilobytes,
    Megabytes,
    Gigabytes,
}

impl FileSize {
    fn find() -> Self {
    let args: Vec<String> = env::args().collect();
    let b_type = args[2].to_lowercase();
    let find = match b_type.as_str() {
        "b" => FileSize::Bytes,
        "kb" => FileSize::Kilobytes,
        "mb" => FileSize::Megabytes,
        "gb" => FileSize::Gigabytes,
        _ => FileSize::Bytes,
    };
    find
    }
}
#[derive(Debug)]
struct Sizes {
    bytes: f64,
    kilobytes: f64,
    megabytes: f64,
    gigabytes: f64,
}
//TODO: 
// get the string input from the terminal enviroment
//parse the number string and use the size string in a match
//calculate the value for each data size and round it accordingly 
//print a debugged version of Sizes with the right values

//ideas and stuff we might need:
//maybe we don't need to split the string if we're usings Args(). we can just do args[1] and parse that
//how do we insert those calculated values back into the struct? i assume we extend format_size()
//maybe we create another enum? 

impl Sizes {
    fn new(bytes: f64, kilobytes: f64, megabytes: f64, gigabytes: f64) -> Self {
        Sizes {
            bytes: bytes,
            kilobytes: kilobytes,
            megabytes: megabytes,
            gigabytes: gigabytes,
        }
    }
}
//have to convert whatever size to bytes. multiply by how many bytes it takes to make up that size. we need to make sure we can divide by every size
fn format_size() {
    let args: Vec<String> = env::args().collect();
    let mut parse = args[1].parse::<f64>().expect("no String number found");
    let find = FileSize::find();
    let sizes = match find {
        FileSize::Bytes => parse,
        FileSize::Kilobytes => parse * 1000.0,
        FileSize::Megabytes => parse * 1_000_000.0,
        FileSize::Gigabytes => parse * 1_000_000_000.0,
    };
    let result = Sizes::new(
        sizes,
        sizes / 1000.0,
        sizes / 1_000_000.0,
        sizes / 1_000_000_000.0,
    );
    println!("{:?}", result);
}


fn main() {
    format_size()
}
