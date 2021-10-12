mod lib;
use lib::btree::{Tree,TreeNode};  

use std::{
    io::{self, BufRead, BufReader, Seek},
    fs::File,
};

use std::io::prelude::*;
use std::io::SeekFrom;

use std::io::Read;
use std::io::Write; // bring trait into scope
//use std::fs;
use std::fs::OpenOptions;

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
    let mut bt = Tree::new();
    bt.addNode("a".to_string(),0);

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

                let mut c41 : Vec<&str> = vec[4].split(" ").collect(); 
                for v41 in c41 {
                    println!("v41 {} -> {}",&vec[4],&v41);
                    bt.add_root(v41.to_string().clone(), pos);
                }

                let mut c47 : Vec<&str> = vec[7].split(" ").collect(); 
                for v47 in c47 {
                    println!("{} -> {}",&vec[7],&v47);
                    bt.add_root(v47.to_string().clone(), pos);
                }   

                println!("_____________________________________________________________________________________________");

            }

            Err(err) => {
                println!("err {}",err);
                break;
            }
        }
    }
    let encoded: Vec<u8> = bincode::serialize(&bt).unwrap();

    let mut file = OpenOptions::new()
    .write(true)
    .create(true)
    // either use ? or unwrap since it returns a Result
    .open("/Users/olivier/dev/rust/btreecomp/btree-1000.bt")?;
    file.write_all(&encoded);
    Ok(())
}
