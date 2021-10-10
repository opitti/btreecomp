use std::{
    io::{self, BufRead, BufReader, Seek},
    fs::File,
};

use std::io::prelude::*;
use std::io::SeekFrom;

fn showline(pos:u64) -> io::Result<()> {

    let mut f2 = BufReader::new(File::open("adresses-782.csv")?);
    // move the cursor 42 bytes from the start of the file
    f2.seek(SeekFrom::Start(pos))?;
    let res = &mut String::new();
    f2.read_line(res)?;
    println!("Showline => {}",res);
    Ok(())
}

fn main() -> io::Result<()> {

    let mut f = BufReader::new(File::open("adresses-782.csv")?);

    loop {
        let mut res = &mut String::new();
        let pos = f.stream_position()?;
        match f.read_line(res) {
            Ok(n) =>{
                if n==0{
                    break;
                }
                println!("bytes read {} {}",res,n);
                println!("pos {}",pos);
                let mut split = res.split(";");
                let vec: Vec<&str> = split.collect();
                //println!("{} ->{} . {} . {} . {}",pos,vec[2],vec[4],vec[6],vec[7]);
                println!("{} ->{} . {}",pos,vec[4],vec[7]);
                println!("_____________________________________________________________________________________________");

            }

            Err(err) => {
                println!("err {}",err);
                break;
            }
        }
    }
    showline(331);
    Ok(())
}