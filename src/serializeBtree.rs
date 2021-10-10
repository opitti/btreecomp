use rand::Rng;
use time::Instant;
use time::Duration;

use serde::{Serialize, Deserialize};

use std::io::Read;
use std::io::Write; // bring trait into scope
//use std::fs;
use std::fs::OpenOptions;

use std::{
    io::{self, BufRead, BufReader, Seek},
    fs::File,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TreeNode {
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
    val: u32,
}

impl TreeNode {
    pub fn new(u: u32) -> Self {
        TreeNode {
            left: None,
            right: None,
            val: u,
        }
    }

    pub fn addNode(&mut self, valeur: u32) {
        let vv = valeur;
        //println!("valeur {}, self.val {}", valeur, self.val);
        if valeur > self.val {
            match &mut self.right {
                None => {
                    //println!("R: {:?}, {}", self, valeur);
                    self.right = Some(Box::new(TreeNode::new(vv)))
                }

                Some(rg) => {
                    //self.right.as_ref().map(|v| v.addNode(vv));
                    rg.addNode(vv);
                }
            }
        } else {
            if valeur < self.val {
                match &mut self.left {
                    None => {
                        //println!("L: {:?}, {}", self, valeur);
                        self.left = Some(Box::new(TreeNode::new(vv)));
                        //println!("after L: {:?}, {}", self, valeur);
                    }

                    Some(lf) => {
                        //self.left.map(|mut v|v.addNode(vv));
                        lf.addNode(vv);
                    }
                }
            }
        }
    }/* end addNode*/

    pub fn findnb(&self, valeur:u32) -> u32 {

        if self.val == valeur {
            return valeur;
        } else {
            if valeur > self.val {
                match &self.right {
                    None => 0,
                    Some(r) => return r.findnb(valeur),
                };
            }else{
               if valeur < self.val {
                   match &self.left {
                       None => 0,
                       Some(l) => return l.findnb(valeur),
                   };
                }
            }
        }
        return 0;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Tree {
    root: Option<Box<TreeNode>>,
}

impl Tree {
    pub fn new() -> Self {
        Tree { root: None }
    }

    pub fn addNode(&mut self, val: u32) {
        match self.root {
            None => {
                self.root = Some(Box::new(TreeNode::new(val)));
            }

            Some(_) => (),
        }
    }

    pub fn add_root(&mut self, valeur: u32) {
        match &mut self.root {
            None => {}
            Some(v) => {
                v.addNode(valeur);
            }
        }
    }

    pub fn find_from_root(&self, valeur: u32) -> u32 {

        match &self.root {
            None => 0,
            Some(root) => return root.findnb(valeur),
        }
    }
}

fn main() -> io::Result<()> {
    let mut t = Tree::new();

    //let v = vec![90, 18, 70, 1, 66, 78, 9,71,172,2,3];
    let mut rng = rand::thread_rng();
    let v: Vec<u32> = (0..1000000).map(|_| rng.gen_range(0..2000000)).collect();
    //let v: Vec<u32> = (0..1000).map(|_| rng.gen_range(0..2000000)).collect();


    let mut first = true;
    for i in &v {
        if first {
            t.addNode(*i);
            first = false;
        } else {
            //println!("{}", i);
            t.add_root(*i);
        }
    }
    let res = t.find_from_root(172);
    println!("{:?}", res);

    let  mut sum = Duration::nanoseconds(1);
    for i in &v {
        let start = Instant::now();
        let res = t.find_from_root(*i);
        let end = start.elapsed();
        sum = sum + end;
        //println!("{:?} => {:?}", res,end);
    }
    println!("sum {:?} secondes , moyenne nanos {:?}",sum.as_seconds_f32() ,sum.subsec_nanoseconds() as usize / v.len());
    let encoded: Vec<u8> = bincode::serialize(&t).unwrap();

    let mut file = OpenOptions::new()
    .write(true)
    // either use ? or unwrap since it returns a Result
    .open("/Users/olivierpittiglio/dev/mBase/btree/btree-1000.bt")?;
    file.write_all(&encoded);

    // ---

    let mut file2 = OpenOptions::new()
    .read(true)
    // either use ? or unwrap since it returns a Result
    .open("/Users/olivierpittiglio/dev/mBase/btree/btree-1000.bt")?;

    let mut decodedVec: Vec<u8> = vec![];
    file2.read_to_end(&mut decodedVec);

    let decoded: Tree = bincode::deserialize(&decodedVec[..]).unwrap();

    let  mut sum2 = Duration::nanoseconds(1);
    for i2 in &v {
        let start = Instant::now();
        let res = decoded.find_from_root(*i2);
        let end = start.elapsed();
        sum2 = sum2 + end;
        //println!("{:?} => {:?}", res,end);
    }
    println!("sum2 {:?} secondes , moyenne nanos {:?}",sum2.as_seconds_f32() ,sum2.subsec_nanoseconds() as usize / v.len());

    Ok(())

}