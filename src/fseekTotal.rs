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

    //let mut f = BufReader::new(File::open("adresses-france.csv")?);
    let mut f = BufReader::new(File::open("adr2.csv")?);
    let mut bt = Tree::new();
    bt.addNode("a".to_string(),0);
    let mut bcltot: u64 = 0;

    loop {
        let mut res = &mut String::new();
        let pos = f.stream_position()?;
        match f.read_line(res) {
            Ok(n) =>{
                if n==0{
                    break;
                }
                //println!("bytes read {} {}",res,n);
                //println!("pos {}",pos);
                let mut split = res.split(";");
                let vec: Vec<&str> = split.collect();
                //println!("{} ->{} . {} . {} . {}",pos,vec[2],vec[4],vec[6],vec[7]);
                //println!("{} ->{} . {}",pos,vec[4],vec[7]);

                let mut c41 : Vec<&str> = vec[4].split(" ").collect();
                //println!("c41 len = {}",c41.len());
                let bcl41 = 1;
                for v41 in c41 {
                    //println!("v41 {} -> {} / {}",&vec[4],&v41,bcl41);
                    bt.add_root(v41.to_string().clone(), pos);
                }

                let mut c47 : Vec<&str> = vec[7].split(" ").collect();
                for v47 in c47 {
                    //println!("{} -> {}",&vec[7],&v47);
                    bt.add_root(v47.to_string().clone(), pos);
                }

                //println!("_____________________________________________________________________________________________");

            }
            // 2434865 
            Err(err) => {
                println!("err {}",err);
                break;
            }
        }
        bcltot = bcltot + 1;
        if (bcltot % 30000) == 0{
            println!("enreg traite : {}",bcltot);
        }
    }
    let encoded: Vec<u8> = bincode::serialize(&bt).unwrap();

    let mut file = OpenOptions::new()
    .write(true)
    .create(true)
    // either use ? or unwrap since it returns a Result
    .open("btree-total.bt")?;
    file.write_all(&encoded);
    Ok(())
}
