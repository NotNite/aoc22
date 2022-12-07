use crate::puzzle::Puzzle;
use std::collections::HashMap;
use vfs::{MemoryFS, VfsPath};

pub struct DaySeven;

fn calc_full_path(thingy: VfsPath) -> String {
    let mut parts = Vec::new();
    parts.push(thingy.filename());

    let mut parent = thingy.parent();
    while let Some(parent2) = parent {
        parts.push(parent2.filename());
        parent = parent2.parent();
    }

    parts.reverse();
    parts.join("/")
}

fn parse_input(input: String) -> (VfsPath, HashMap<String, usize>) {
    let mut root: VfsPath = MemoryFS::new().into();
    let lines = input.lines(); // skip the first cd /
    let mut file_to_size_hashmap = HashMap::new();

    for line in lines {
        //println!("line: {}", line);
        if line.starts_with('$') {
            let mut split = line.split(' ');
            let _ = split.next().unwrap();
            let command = split.next().unwrap();

            if command == "cd" {
                let path = split.next().unwrap();
                let path = path.trim_start_matches('/');

                if path == ".." {
                    root = root.parent().unwrap();
                } else {
                    root = root.join(path).unwrap();
                    if !root.exists().unwrap() {
                        root.create_dir().unwrap();
                    }
                }
            }
        } else {
            let mut split = line.split(' ');
            let size_or_dir = split.next().unwrap();
            let name = split.next().unwrap();

            if size_or_dir == "dir" {
                let file = root.join(name).unwrap();
                file.create_dir().unwrap();
            } else {
                let size = size_or_dir.parse::<usize>().unwrap();
                let file = root.join(name).unwrap();
                file.create_file().unwrap();
                // im not filling my filesystem with Data to calc size lol
                let full_path = calc_full_path(file.clone());
                file_to_size_hashmap.insert(full_path, size);
            }
        }
    }

    (root.root(), file_to_size_hashmap)
}

fn deez(path: VfsPath, file_to_size: HashMap<String, usize>, counter: &mut usize) -> usize {
    let mut size_of_this_path = 0;

    for entry in path.read_dir().unwrap() {
        if entry.is_dir().unwrap() {
            let dir_size = deez(entry.clone(), file_to_size.clone(), counter);
            if dir_size <= 100000 {
                *counter += dir_size;
            }
            size_of_this_path += dir_size;
            //println!("DIR: {} {}", entry.filename(), dir_size);
        } else {
            let full_path = calc_full_path(entry.clone());
            let file_size = file_to_size.get(&full_path).unwrap();
            /*println!(
                "FILE: {} {} {}",
                path.filename(),
                entry.filename(),
                file_size
            );*/
            size_of_this_path += file_size;
        }
    }

    size_of_this_path
}

fn lmao(
    path: VfsPath,
    file_to_size: HashMap<String, usize>,
    needed_space: usize,
    smallest_dir: &mut usize,
) -> usize {
    let mut size_of_this_path = 0;

    for entry in path.read_dir().unwrap() {
        if entry.is_dir().unwrap() {
            let dir_size = lmao(
                entry.clone(),
                file_to_size.clone(),
                needed_space,
                smallest_dir,
            );
            size_of_this_path += dir_size;
        } else {
            let full_path = calc_full_path(entry.clone());
            let file_size = file_to_size.get(&full_path).unwrap();
            size_of_this_path += file_size;
        }
    }

    if size_of_this_path >= needed_space && size_of_this_path < *smallest_dir {
        *smallest_dir = size_of_this_path;
    }

    size_of_this_path
}

impl Puzzle for DaySeven {
    fn one(&self, input: String) -> String {
        let (vfs, file_to_size) = parse_input(input);

        let mut counter = 0;
        let root_dir_size = deez(vfs, file_to_size, &mut counter);
        //println!("ROOT DIR: {}", root_dir_size);
        if root_dir_size <= 100000 {
            counter += root_dir_size;
        }

        counter.to_string()
    }

    fn two(&self, input: String) -> String {
        let (vfs, file_to_size) = parse_input(input);

        let mut counter = 0;
        let root_dir_size = deez(vfs.clone(), file_to_size.clone(), &mut counter);

        let free_space = 70000000 - root_dir_size;
        let needed_space = 30000000 - free_space;

        let mut smallest_dir = 70000000;
        let _ = lmao(vfs, file_to_size, needed_space, &mut smallest_dir);

        smallest_dir.to_string()
    }
}
