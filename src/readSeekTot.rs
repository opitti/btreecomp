mod lib;
use lib::btree::{Tree};

use std::{
    io::{self, BufRead, BufReader, Seek},
    fs::File,
};

//use std::io::prelude::*;
use std::io::SeekFrom;

use std::io::Read;
//use std::io::Write; // bring trait into scope
//use std::fs;
use std::fs::OpenOptions;
//use array_tool::vec::Intersect;

use time::Instant;
//use time::Duration;
use std::collections::HashSet;
extern crate quickersort;

fn main() -> io::Result<()> {

    let s1 = Instant::now();
    let mut file2 = OpenOptions::new()
    .read(true)
    // either use ? or unwrap since it returns a Result
    .open("/Users/olivierpittiglio/dev/mBase/btree-total.bt")?;
    let e1 = s1.elapsed();
    println!("lecture de l'index {:?} ",e1);

    let s2 = Instant::now();
    #[allow(non_snake_case)]
    let mut decodedVec: Vec<u8> = vec![];
    //#[allow(unused_must_use)]
    file2.read_to_end(&mut decodedVec)?;
    let decoded: Tree = bincode::deserialize(&decodedVec[..]).unwrap();
    let e2 = s2.elapsed();
    println!("decodage de l'index dans un Tree {:?} ",e2);

    //  Vieux Moulin
    println!("read decoded end ");

    // 1er recherche
    let s3 = Instant::now();
    let res4 = decoded.find_from_root("Vieux".to_string());
    let e3 = s3.elapsed();
    //println!("{:?}", res4);
    println!("nombre de resultat {:?}  temps de calcul {:?}", res4.unwrap().len(),e3);
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
/*  working method but take time
    println!("sorting res2");
    let mut r = res2.unwrap().to_vec();
    quickersort::sort(&mut r[..]);
    println!("sorting res4");
    let mut r2 = res4.unwrap().to_vec();
    quickersort::sort(&mut r2[..]);

    println!("before intersection");
    //let resinters = res2.unwrap().intersect(res4.unwrap().to_vec());
    let resinters = r.intersect(r2);
*/
let start = Instant::now();
    let set_res2: HashSet<_> = res2.unwrap().into_iter().collect();
    let set_res4: HashSet<_> = res4.unwrap().into_iter().collect();
    let resinters = set_res2.intersection(&set_res4);
    let end = start.elapsed();
    println!("calcul de l'intersections {:?} ",end);
    for resinter in resinters{
        // --> sur l'imac
        let mut f2 = BufReader::new(File::open("/Volumes/olivier2/dev/adresses/ORIGIN/adresses-france.csv")?);
        // --> sur le lenovo : let mut f2 = BufReader::new(File::open("adr2.csv")?);
        // move the cursor 42 bytes from the start of the file

        f2.seek(SeekFrom::Start(**resinter))?;
        let r2 = &mut String::new();
        f2.read_line(r2)?;
        println!("Showline inter => {}",r2);

    }

    Ok(())
}
