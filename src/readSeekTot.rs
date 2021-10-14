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
use array_tool::vec::Intersect;

use time::Instant;
use time::Duration;

extern crate quickersort;

fn main() -> io::Result<()> {
    let start = Instant::now();
    let mut file2 = OpenOptions::new()
    .read(true)
    // either use ? or unwrap since it returns a Result
    .open("/Users/olivierpittiglio/dev/mBase/btree-total.bt")?;
    let mut decodedVec: Vec<u8> = vec![];
    file2.read_to_end(&mut decodedVec);
    let decoded: Tree = bincode::deserialize(&decodedVec[..]).unwrap();

    println!("read decoded end ");
    let res4 = decoded.find_from_root("Barre".to_string());
    //println!("{:?}", res4);
    println!("{:?}", res4.unwrap().len());
/*
    for pos in res4.unwrap() {
        let mut f2 = BufReader::new(File::open("adresses-78.csv")?);
        // move the cursor 42 bytes from the start of the file
        f2.seek(SeekFrom::Start(*pos))?;
        let res = &mut String::new();
        f2.read_line(res)?;
        println!("Showline => {}",res);
    }
*/
    let res2 = decoded.find_from_root("Moulin".to_string());
    //println!("{:?}", res2);
    println!("{:?}", res2.unwrap().len());
/*
    for pos in res2.unwrap() {
        let mut f2 = BufReader::new(File::open("adresses-78.csv")?);
        // move the cursor 42 bytes from the start of the file
        f2.seek(SeekFrom::Start(*pos))?;
        let r2 = &mut String::new();
        f2.read_line(r2)?;
        println!("Showline => {}",r2);
    }
*/
    println!("sorting res2");
    let mut r = res2.unwrap().to_vec();
    quickersort::sort(&mut r[..]);
    println!("sorting res4");
    let mut r2 = res4.unwrap().to_vec();
    quickersort::sort(&mut r2[..]);

    println!("before intersection");
    //let resinters = res2.unwrap().intersect(res4.unwrap().to_vec());
    let resinters = r.intersect(r2);
    let end = start.elapsed();
    println!("sum {:?} ",end);
    for resinter in resinters{
        let mut f2 = BufReader::new(File::open("/Volumes/olivier2/dev/adresses/ORIGIN/adresses-france.csv")?);
        // move the cursor 42 bytes from the start of the file
        f2.seek(SeekFrom::Start(resinter))?;
        let r2 = &mut String::new();
        f2.read_line(r2)?;
        println!("Showline inter => {}",r2);

    }

    Ok(())
}
