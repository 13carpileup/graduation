#![allow(unused)]

use super::get_classes;
use crate::structs::User;

use std::cmp;
use std::collections::{ HashMap, HashSet, VecDeque };

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
pub async fn get_colourings(students: Vec<User>) -> Vec<(User, String)> {
    let mut out = vec![];
    let mut colour_map: HashMap<Vec<String>, String> = HashMap::new();
    let mut v_space: HashSet<Vec<String>> = HashSet::new();

    let mut adj_list: HashMap<Vec<String>, Vec<Vec<String>>> = HashMap::new();

    for student in students {
        let classes = super::get_classes(student.id).await;

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

    let colour_prime = String::from("#ffffff");

    let elem = v_space.iter().next().unwrap().clone();
    traverse_path(&adj_list, &mut visited, &mut colour_map, elem, colour_prime);


    
    out
}

fn traverse_path(adj_list: &HashMap<Vec<String>, Vec<Vec<String>>>, visited: &mut HashMap<Vec<String>, bool>, colour_map: &mut HashMap<Vec<String>, String>, prime_node: Vec<String>, prime_colour: String) {
    let mut deq: VecDeque<Vec<String>> = VecDeque::new();
    deq.push_back(prime_node);

    while deq.len() > 0 {
        let current = deq.pop_front().unwrap();
        let e = visited.entry(current.clone()).or_insert(true);
        *e = true;

        while let Some(entry) = adj_list.get(&current) {
            for connection in entry {
                match visited.get(connection) {
                    Some(b) => {
                        if !*b {
                            deq.push_back((*connection.clone()).to_vec());
                        }
                    },
                    None => println!("ERROR"),
                } 
            }
        }

    }
}