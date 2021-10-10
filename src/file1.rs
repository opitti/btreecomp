use std::{
    io::{self, BufRead, BufReader, Seek},
    fs::File,
};

use std::io::prelude::*; 
use std::io::SeekFrom;

fn main() -> io::Result<()> {
    let mut f = BufReader::new(File::open("foo.txt")?);

    let before = f.stream_position()?;
    println!("before {}",&before);
    f.read_line(&mut String::new())?;
    f.read_line(&mut String::new())?;
    let after = f.stream_position()?;
    println!("after {}",&after);
    println!("The first line was {} bytes long", after - before);

    let mut f2 = BufReader::new(File::open("foo.txt")?);
    // move the cursor 42 bytes from the start of the file
    f2.seek(SeekFrom::Start(after))?;
    let res = &mut String::new();
    f2.read_line(res)?;
    println!("res : {} ",res);



    Ok(())
}