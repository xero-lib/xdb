use regex::Regex;
use std::{fs::File, os::unix::prelude::FileExt};

fn main() {
    let num_regex: Regex = Regex::new(r"\d+").unwrap();

    let mut procs: Vec<u32> = Vec::new();
    let mut names: Vec<String> = Vec::new();
    let mut maps: Vec<String> = Vec::new();
    let mut mems: Vec<File> = Vec::new();

    for dir in std::fs::read_dir("/proc").unwrap() {
        let fname = dir.as_ref().unwrap().file_name();
        match num_regex.captures(fname.to_str().unwrap()) {
            Some(_) => procs.push(fname.to_str().unwrap().parse::<u32>().unwrap()),
            None => (),
        }
    }

    for proc in procs.iter() {
        mems.push(match File::open(format!("/proc/{}/mem", &proc)) {
            Ok(file) => file,
            Err(_) => {
                println!("Could not open mem files. Please run as root.");
                std::process::exit(1);
            }
        });

        maps.push(std::fs::read_to_string(format!("/proc/{}/maps", &proc)).unwrap());
        names.push(
            std::fs::read_to_string(format!("/proc/{}/comm", proc))
                .expect("Couldn't read comm file")
                .trim()
                .to_string(),
        );
    }

    for (loc, _id) in procs.iter().enumerate() {
        for line in maps[loc].split('\n').map(ToString::to_string) {
            let data = &line
                .split_whitespace()
                .map(ToString::to_string)
                .collect::<Vec<_>>();
            if data.len() < 2 {
                continue;
            };
            let [range, perms] = [&data[0], &data[1]];
            if perms.chars().collect::<Vec<char>>()[0] != 'r' {
                continue;
            };
            let m_ranges = &range
                .split('-')
                .map(ToString::to_string)
                .collect::<Vec<_>>();
            if m_ranges.len() < 2 {
                continue;
            };
            let [start, end] = [
                u64::from_str_radix(&m_ranges[0], 16).unwrap(),
                u64::from_str_radix(&m_ranges[1], 16).unwrap(),
            ];
            let mut byte_dump: [u8; 1] = [0];
            let mut full_dump = Vec::<u8>::new();
            full_dump.reserve((end - start) as usize);

            for index in 0..(end - start) {
                let _ = mems[loc].read_exact_at(&mut byte_dump, start + index);
                full_dump.push(*byte_dump.first().unwrap());
            }

            //let out = mem_dump_buffer.concat().iter().map(|c| *c as char).collect::<String>();
            // out = full_dump.iter().map(|c| *c as char).collect::<String>();
            let out = full_dump
                .iter()
                .map(|c| format!("{:X}", c))
                .collect::<String>();
            print!("{}", out);
        }
    }
}
