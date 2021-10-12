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
    val: String,
    index: Vec<u64>
}

impl TreeNode {
    pub fn new(u: String, ind:u64) -> Self {
        TreeNode {
            left: None,
            right: None,
            val: u,
            index: vec![ind]
        }
    }

    pub fn addNode(&mut self, valeur: String, ind:u64) {
        let vv = valeur.clone();
        //println!("add  valeur {}, ", &valeur);
        match valeur {
            d if d > self.val => {
                match &mut self.right {
                    None => {
                        //println!("R: {:?}, {}", &self, &vv);
                        self.right = Some(Box::new(TreeNode::new(vv, ind)))
                    }
                    Some(rg) => {
                        //self.right.as_ref().map(|v| v.addNode(vv));
                        rg.addNode(vv, ind);
                    }
                }

            },
            d if d < self.val => {
                match &mut self.left {
                    None => {
                        //println!("L: {:?}, {}", self, valeur);
                        self.left = Some(Box::new(TreeNode::new(vv, ind)));
                        //println!("after L: {:?}, {}", self, valeur);
                    }

                    Some(lf) => {
                        //self.left.map(|mut v|v.addNode(vv));
                        lf.addNode(vv,ind);
                    }                }

            },
            d if d == self.val => {
                //println!("FIND SAME");
                self.index.push(ind);
            },
            _ => {}
        }
    }/* end addNode*/

    pub fn findnb(&self, valeur:String) -> Option<&Vec<u64>>{

        if self.val == valeur {
            return Some(&self.index);
        } else {
            if valeur > self.val {
                match &self.right {
                    None => {},
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
        return None;
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

    pub fn addNode(&mut self, val: String, ind:u64) {
        match self.root {
            None => {
                self.root = Some(Box::new(TreeNode::new(val, ind)));
            }
            Some(_) => (),
        }
    }

    pub fn add_root(&mut self, valeur: String, ind:u64) {
        match &mut self.root {
            None => {}
            Some(v) => {
                v.addNode(valeur, ind);
            }
        }
    }

    pub fn find_from_root(&self, valeur: String) -> Option<&Vec<u64>> {

        match &self.root {
            None => None,
            Some(root) => return root.findnb(valeur),
        }
    }
}