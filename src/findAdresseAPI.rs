mod lib;

#[macro_use] extern crate rocket;
use rocket::tokio::time::{sleep, Duration};
use rocket::State;
use rocket::response::status::NotFound;
use rocket::serde::{Serialize, json::Json};


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


#[derive(Serialize)]
pub struct MyResult{
    res: Vec<String>
}
impl MyResult {
    pub fn new()->Self{
        MyResult{
            res: Vec::new()
        }
    }
}


pub struct DecodedTree{
    decoded:Tree
}

impl DecodedTree {

    pub fn initTree() -> io::Result<DecodedTree> {
        let s1 = Instant::now();
        let mut file2 = OpenOptions::new()
        .read(true)
        .open("/Users/olivierpittiglio/dev/mBase/btree-total.bt")?;
        //.open("/home/olivier/dev/rust/btreecompdata/btree-total.bt")?;
        let e1 = s1.elapsed();
        println!("lecture de l'index {:?} ",e1);

        let s2 = Instant::now();
        #[allow(non_snake_case)]
        let mut decodedVec: Vec<u8> = vec![];
        file2.read_to_end(&mut decodedVec)?;
        let decoded: Tree = bincode::deserialize(&decodedVec[..]).unwrap();
        let e2 = s2.elapsed();
        println!("decodage de l'index dans un Tree {:?} ",e2);
        println!("read decoded end ");
        return Ok(DecodedTree{
            decoded: decoded
        })
    }

    pub fn find(&self,val: String) -> Option<&Vec<u64>> {
        let s3 = Instant::now();
        let res4 = self.decoded.find_from_root(val);
        let e3 = s3.elapsed();
        println!("nombre de resultat {:?}  temps de calcul {:?}", res4.unwrap().len(),e3);
        return res4;
    }

    pub fn findfor3(&self,str1:String,str2:String,str3:String) -> io::Result<Vec<String>> {

        let res2 = self.find(str1);
        let res4 = self.find(str2);
        let res5 = self.find(str3);
        //let res6 = decoded.find("Vieux".to_string());

        let start = Instant::now();
        let set_res2: HashSet<_> = res2.unwrap().into_iter().collect();
        let set_res4: HashSet<_> = res4.unwrap().into_iter().collect();
        let set_res5: HashSet<_> = res5.unwrap().into_iter().collect();

        let resinters = set_res2.intersection(&set_res4);

        let set_res6= resinters.copied().collect();
        let resinters2  = set_res5.intersection(&set_res6);
        let end = start.elapsed();
        println!("calcul de l'intersections {:?} ",end);

        let mut res: Vec<String> = Vec::new();

        for resinter in resinters2{
            let mut f2 = BufReader::new(File::open("/Volumes/olivier2/dev/adresses/ORIGIN/adresses-france.csv")?);
            //let mut f2 = BufReader::new(File::open("/home/olivier/dev/rust/btreecompdata/adresses-france.csv")?);
            f2.seek(SeekFrom::Start(**resinter))?;
            let r2 = &mut String::new();
            f2.read_line(r2)?;
            res.push(r2.clone());
        }
        return Ok(res);
    }

    pub fn findfor2(&self,str1:String,str2:String,) -> io::Result<MyResult> {

        let res2 = self.find(str1);
        let res4 = self.find(str2);

        let start = Instant::now();
        let set_res2: HashSet<_> = res2.unwrap().into_iter().collect();
        let set_res4: HashSet<_> = res4.unwrap().into_iter().collect();
        let resinters = set_res2.intersection(&set_res4);

        let end = start.elapsed();
        println!("calcul de l'intersections {:?} ",end);

        //let mut res: Vec<String> = Vec::new();
        let mut outres = MyResult::new();
        let mut somme_des_seek: f64 = 0.0;

        let start_file = Instant::now();
        let mut f2 = BufReader::new(File::open("/Volumes/olivier2/dev/adresses/ORIGIN/adresses-france.csv")?);
        //let mut f2 = BufReader::new(File::open("/home/olivier/dev/rust/btreecompdata/adresses-france.csv")?);
        let end_file = start_file.elapsed();
        println!("open file time : {}",end_file.as_seconds_f64());
    
        for resinter in resinters{

            let start_file_seek = Instant::now();
            f2.seek(SeekFrom::Start(**resinter))?;
            let end_file_seek = start_file_seek.elapsed();
            println!("file seek time : {}",end_file_seek.as_seconds_f64());
            somme_des_seek += end_file.as_seconds_f64();

            let r2 = &mut String::new();
            f2.read_line(r2)?;
            outres.res.push(r2.clone());
        }
        println!("somme des file seek {}",somme_des_seek);
        return Ok(outres);
    }

}



#[get("/find/<str1>/<str2>")]
fn index(str1:String, str2:String,decoded: &State<DecodedTree>) ->  Json<MyResult> {
    let res = decoded.findfor2(str1,str2).unwrap();
    println!("find nymber of result: {}",res.res.len());
    return Json(res);
}

#[rocket::main]
async fn main()  {

    let decoded = DecodedTree::initTree().unwrap();

    println!("START API");
    rocket::build()
    .manage(decoded)
    .mount("/", routes![index])
    .launch()
    .await;

    /*
    for res in decoded.findfor3("Lot".to_string(), "Eau".to_string(), "Chateau".to_string()).unwrap() {
        println!("Solution ==> {}",res);
    }
    */
    //Ok(())
}
