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

fn main() -> io::Result<()> {

    let mut file2 = OpenOptions::new()
    .read(true)
    // either use ? or unwrap since it returns a Result
    .open("/Users/olivier/dev/rust/btreecomp/btree-1000.bt")?;
    let mut decodedVec: Vec<u8> = vec![];
    file2.read_to_end(&mut decodedVec);
    let decoded: Tree = bincode::deserialize(&decodedVec[..]).unwrap();

    let res = decoded.find_from_root("Moulin".to_string());
    println!("{:?}", res);
    
    Ok(())
}