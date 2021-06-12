use std::fs;
use std::io;
use std::io::Read;

fn main() {
    let content = read_content_from_file().unwrap();
    println!("File content: {}", content);

    let content = read_content_from_file1("hello.txt").unwrap();
    println!("File content: {}", content);

    let content = read_content_from_file2().unwrap();
    println!("File content: {}", content);

    let content = read_content_from_file3().unwrap();
    println!("File content: {}", content);

    open_file_match();
}

fn open_file_match() {
    let f = fs::File::open("hello.txt");
    match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => match fs::File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Error: {:?}", e),
            },
            err => panic!("Problem opening the file: {:?}", err),
        },
    };
}

fn read_content_from_file() -> Result<String, io::Error> {
    let file = fs::File::open("hello.txt");

    let mut file = match file {
        Ok(f) => f,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match file.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_content_from_file1(p: &str) -> Result<String, io::Error> {
    let mut file = fs::File::open(p)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

fn read_content_from_file2() -> Result<String, io::Error> {
    let mut s = String::new();

    fs::File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn read_content_from_file3() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
