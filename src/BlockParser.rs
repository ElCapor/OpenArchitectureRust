use crate::TypeExtensions::StringExt;
use std::{
    char, fs::File, io::{prelude::*, BufReader}, path::{Path, PathBuf}
};

/// Simple implementation of a class that splits an assembly files into blocks
pub struct BlockParser
{
    pub blocks : Vec<Vec<String>>, // a block is a just a list of strings
}

impl  BlockParser{
    // strip the comments from a block
    pub fn strip_comments(&self, block : &Vec<String>) -> Vec<String>
    {
        let mut ret : Vec<String> = Vec::new();
        let mut is_comment : bool = false;
        for line in block.iter()
        {
            if line.chars().nth(0).unwrap()== ';' && line.chars().last().unwrap() == ';'
            {
                continue;
            } else if line.get_char_count(';') == 2{
                ret.push(line.split(";").next().unwrap().trim().to_string());
                continue;
            } else if line.get_char_count(';') == 1{
                if !is_comment { is_comment = true;}
                else {is_comment = false;}
                continue;
            } else {
                ret.push(line.to_string())
            }
        }

        for l in ret.iter()
        {
            println!("{}", l);
        }
        return ret;
    }

    pub fn new(&mut self, input_path :std::path::PathBuf) {
        let path = PathBuf::from(input_path);
        let file = File::open(path).expect("no such file");
        let buf = BufReader::new(file);
        let data : Vec<String> = buf.lines().map(|l| l.unwrap()).collect();

        let mut blocks :Vec<Vec<String>> = Vec::new();
        let mut current_block :Vec<String> = Vec::new();

        for line in data.iter(){
            if line.trim() == "" {
                blocks.push(self.strip_comments(&current_block));
                current_block.clear();
            } else {
                current_block.push(line.trim().to_string());
            }
        }
        if !current_block.is_empty()
        {
            blocks.push(self.strip_comments(&current_block));
        }

        self.blocks = blocks;
    }
}
