#![allow(unused)]

use super::get_classes;
use super::helper::create_adj_list;
use crate::structs::User;

use std::cmp;
use std::collections::{ HashMap, HashSet, VecDeque };
use rand::Rng; 

/// takes a list of students, returns a kind of dict pointing to their colours
pub async fn get_colourings(students: Vec<User>) -> Vec<(User, (i32, i32, i32))> {
    let mut out = vec![];
    let mut v_space: HashSet<Vec<String>> = HashSet::new();
    let mut student_map: HashMap<User, Vec<String>> = HashMap::new();

    let mut adj_list: HashMap<Vec<String>, Vec<Vec<String>>> = HashMap::new();

    let res = create_adj_list(&students).await;
    v_space = res.0;
    adj_list = res.1;
    student_map = res.2;

    // traverse graph
    let mut working_colour_map: HashMap<Vec<String>, Vec<(i32, i32, i32)>> = HashMap::new();

    let repetitions = 2;

    let collected_v_space: Vec<Vec<String>> = v_space.clone().into_iter().collect();
    for i in 0..repetitions {
        let elem = collected_v_space[i].clone();

        let colour_map = returns_colours(&v_space, &adj_list, elem);
        for val in &collected_v_space {
            let ent = working_colour_map.entry(val.to_vec()).or_insert(vec![]);
            ent.push(*colour_map.get(val).unwrap());
        }
    }

    let mut averaged_colour_map: HashMap<Vec<String>, (i32, i32, i32)> = HashMap::new();

    for val in &collected_v_space {
        let mut colour_total = (0, 0, 0);
        let colours: &Vec<(i32, i32, i32)> = working_colour_map.get(val).unwrap();

        for colour in colours {
            colour_total.0 += colour.0;
            colour_total.1 += colour.1;
            colour_total.2 += colour.2;
        }

        colour_total.0 = colour_total.0 / repetitions as i32;
        colour_total.1 = colour_total.1 / repetitions as i32;
        colour_total.2 = colour_total.2 / repetitions as i32;

        averaged_colour_map.insert(val.clone(), colour_total);
    }



    for student in &students {
        let classes = student_map.get(student).unwrap();
        let colour = averaged_colour_map.get(classes).unwrap();

        out.push((student.clone(), *colour));

    }
    
    //println!("Exitting out...");
    out
}

fn returns_colours(v_space: &HashSet<Vec<String>>, adj_list: &HashMap<Vec<String>, Vec<Vec<String>>>, prime_node: Vec<String>) ->  HashMap<Vec<String>, (i32, i32, i32)> {
    let mut colour_map: HashMap<Vec<String>, (i32, i32, i32)> = HashMap::new();
    let mut visited: HashMap<Vec<String>, bool> = HashMap::new();

    for val in v_space {
        visited.insert(val.clone(), false);
    }

    let colour_prime = (0, 0, 0);

    traverse_path(&adj_list, &mut visited, &mut colour_map, prime_node.clone(), colour_prime);

    for set in v_space {
        let resp = visited.get(set).unwrap();
        if !resp {
            println!("Traversing again...");
            traverse_path(&adj_list, &mut visited, &mut colour_map, (*set.clone()).to_vec(), colour_prime);
        }
    }

    colour_map
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
                        colour_map.insert((*connection.clone()).to_vec(), update_colour(*current_colour));
                    }
                },
                None => println!("ERROR"),
            } 
        }
        

    }
}

fn update_colour(og: (i32, i32, i32)) -> (i32, i32, i32) {
    let delta = 50;
    let mut out = og;
    let num = rand::rng().random_range(0..3);
    //println!("Index is {num}");
    
    if num==0 {
        out.0 = cmp::min(og.0 + delta, 220);
    }

    if num==1 {
        out.1 = cmp::min(og.1 + delta, 220);
    }

    if num==2 {
        out.2 = cmp::min(og.2 + delta, 220);
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