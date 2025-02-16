#![allow(unused)]

use super::get_classes;
use crate::structs::User;

use std::cmp;
use std::collections::{ HashMap, HashSet, VecDeque };
use rand::Rng; 

/// checks the number of differences between class lists
fn check_difference(v1: &Vec<String>, v2: &Vec<String>) -> u64 {
    let mut matches = 0;

    for c1 in v1 {
        for c2 in v2 {
            if c1 == c2 {
                matches += 1;
            }
        }
    }

    let n = cmp::min(v1.len(), v2.len());

    (n - matches).try_into().unwrap()
}

/// takes a list of students, returns a kind of dict pointing to their colours
pub async fn get_colourings(students: Vec<User>) -> Vec<(User, (i32, i32, i32))> {
    let mut out = vec![];
    let mut colour_map: HashMap<Vec<String>, (i32, i32, i32)> = HashMap::new();
    let mut v_space: HashSet<Vec<String>> = HashSet::new();
    let mut student_map: HashMap<User, Vec<String>> = HashMap::new();

    let mut adj_list: HashMap<Vec<String>, Vec<Vec<String>>> = HashMap::new();

    for student in &students {
        let classes = super::get_classes(student.id).await;

        student_map.insert(student.clone(), classes.clone());
        v_space.insert(classes.clone());
        adj_list.insert(classes.clone(), vec![]);
    }

    for arr1 in &v_space {
        for arr2 in &v_space {
            let dif = check_difference(arr1, arr2);

            if dif == 1 {
                let r1 = adj_list.entry(arr1.to_vec()).or_insert(vec![]);
                r1.push(arr2.to_vec());

                let r2 = adj_list.entry(arr2.to_vec()).or_insert(vec![]);
                r2.push(arr1.to_vec());
            }
        }
    }

    // traverse graph
    let mut visited: HashMap<Vec<String>, bool> = HashMap::new();

    for val in &v_space {
        visited.insert(val.clone(), false);
    }

    let colour_prime = (40, 40, 40);

    let elem = v_space.iter().next().unwrap().clone();
    //println!("Initial loop...");
    traverse_path(&adj_list, &mut visited, &mut colour_map, elem.clone(), colour_prime);

    for set in &v_space {
        let resp = visited.get(set).unwrap();
        if !resp {
            println!("Traversing again...");
            traverse_path(&adj_list, &mut visited, &mut colour_map, (*set.clone()).to_vec(), colour_prime);
        }
    }

    //println!("Constructing map...");
    for student in &students {
        
        let classes = student_map.get(student).unwrap();
        let colour = colour_map.get(classes).unwrap();

        if elem == *classes {
            println!("{name}", name = student.name);
        }

        out.push((student.clone(), *colour));
    }
    
    //println!("Exitting out...");
    out
}

fn traverse_path(adj_list: &HashMap<Vec<String>, Vec<Vec<String>>>, visited: &mut HashMap<Vec<String>, bool>, colour_map: &mut HashMap<Vec<String>, (i32, i32, i32)>, prime_node: Vec<String>, prime_colour: (i32, i32, i32)) {
    let mut deq: VecDeque<Vec<String>> = VecDeque::new();
    deq.push_back(prime_node.clone());

    colour_map.insert(prime_node, prime_colour);

    while deq.len() > 0 {
        let current = deq.pop_front().unwrap();
        //println!("LOADING.. {c}", c = current[0]);
        let e = visited.entry(current.clone()).or_insert(true);
        *e = true;

        let entry = adj_list.get(&current).unwrap();

        //println!("Inner loop...");
        let mut i = 0; 
        for connection in entry {            
            match visited.get(connection) {
                Some(b) => {
                    if !*b {
                        i += 1;

                        deq.push_back((*connection.clone()).to_vec());
                        let current_colour = colour_map.get(&current).unwrap();
                        colour_map.insert((*connection.clone()).to_vec(), update_colour(*current_colour, i));
                    }
                },
                None => println!("ERROR"),
            } 
        }
        

    }
}

fn update_colour(og: (i32, i32, i32), index: u64) -> (i32, i32, i32) {
    let delta = 60;
    let mut out = og;
    let num = rand::rng().random_range(0..100);
    //println!("Index is {index}");
    
    if get_bit_at(num, 0) {
    
            out.0 = cmp::min(og.0 + delta, 255);
        
    }

    if get_bit_at( num, 1) {
   
        out.1 = cmp::min(og.1 + delta, 255);
    
    }

    if get_bit_at(num, 2) {
        out.2 = cmp::min(og.2 + delta, 255);
        
    }
    
    out
}

/// gets the bit at position `n`. Bits are numbered from 0 (least significant) to 31 (most significant).
fn get_bit_at(input: u64, n: u8) -> bool {
    if n < 32 {
        input & (1 << n) != 0
    } else {
        false
    }
}