use std::{collections::HashMap, error::Error};

use crate::util;

static test_input: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

struct Dir {
    id: usize,
    parent: Option<usize>,
    name: String,
    files: HashMap<String, usize>,
    dirs: HashMap<String, usize>,
    total_files: usize,
}

impl Dir {
    fn new<S: Into<String>>(id: usize, parent: Option<usize>, name: S) -> Self {
        Self {
            id,
            parent,
            name: name.into(),
            files: HashMap::new(),
            dirs: HashMap::new(),
            total_files: 0,
        }
    }

    fn find_sub_dir(&self, name: &str) -> Option<usize> {
        self.dirs.get(name).map(|x| *x)
    }

    fn add_dir(&mut self, name: &str, id: usize) {
        self.dirs.insert(name.into(), id);
    }

    fn add_file(&mut self, name: &str, size: usize) {
        self.files.insert(name.into(), size);
        self.total_files += size;
    }

    fn total_size(&self, fs: &[Dir]) -> usize {
        let dirs_size: usize = self.dirs.values().map(|id| fs[*id].total_size(fs)).sum();
        self.total_files + dirs_size
    }
}

fn dump(id: usize, fs: &[Dir], indent: usize) {
    for _ in 0..indent {
        print!("  ")
    }
    println!("- {} (dir)", fs[id].name);
    // dump sub dirs
    let mut dirs: Vec<_> = fs[id].dirs.keys().collect();
    dirs.sort();
    for dir_name in dirs {
        let dir = fs[id].dirs.get(dir_name).unwrap();
        dump(*dir, fs, indent + 1);
    }
    // dump files
    let mut files: Vec<_> = fs[id].files.keys().collect();
    files.sort();
    for file_name in files {
        let size = fs[id].files.get(file_name).unwrap();
        for _ in 0..indent + 1 {
            print!("  ")
        }
        println!("- {} (file, size={})", file_name, size)
    }
}

fn parse(input: &str) -> Vec<Dir> {
    let mut fs = Vec::new();
    // add root
    fs.push(Dir::new(0, None, "/"));

    let mut cwd = 0;

    for line in util::read_lines(input) {
        match line {
            "$ ls" => {}
            "$ cd /" => {
                cwd = 0;
            }
            "$ cd .." => {
                cwd = fs[cwd].parent.expect("cd .. with no parent");
            }
            c if c.starts_with("$ cd") => {
                let x: Vec<_> = c.split(" ").collect();
                let name = x[2];

                cwd = fs[cwd].find_sub_dir(name).expect("subdir missing");
            }
            // a dir
            c if c.starts_with("dir") => {
                let x: Vec<_> = c.split(" ").collect();
                let name = x[1];

                let id = fs.len();
                fs[cwd].add_dir(name, id);
                let dir = Dir::new(id, Some(cwd), name);
                fs.push(dir);
            }
            // a file
            _ => {
                let x: Vec<_> = line.split(" ").collect();
                fs[cwd].add_file(x[1], x[0].parse().expect("bad int"));
            }
        }
    }
    fs
}

pub fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let fs = parse(input);
    // dump(0, &fs, 0);
    let size: usize = fs
        .iter()
        .map(|dir| dir.total_size(&fs))
        .filter(|size| *size <= 100000)
        .sum();
    println!("{:?}", size);
    Ok(())
}

pub fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    let fs = parse(input);
    let total_size = fs[0].total_size(&fs);
    let required = 30000000 - (70000000 - total_size);
    println!("required: {}", required);

    let size = fs
        .iter()
        .map(|dir| dir.total_size(&fs))
        .filter(|size| *size >= required)
        .min();

    println!("min: {:?}", size);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let fs = parse(test_input);
        assert_eq!(fs.len(), 4);
    }
}
