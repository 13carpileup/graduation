use crate::structs::User;
use std::cmp;
use std::collections::{ HashMap, HashSet };

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

/// returns (v_space, adj_list, student_map)
pub async fn create_adj_list(students: &Vec<User>) -> (HashSet<Vec<String>>, HashMap<Vec<String>, Vec<Vec<String>>>, HashMap<User, Vec<String>>) {
    let mut v_space: HashSet<Vec<String>> = HashSet::new();
    let mut student_map: HashMap<User, Vec<String>> = HashMap::new();

    let mut adj_list: HashMap<Vec<String>, Vec<Vec<String>>> = HashMap::new();

    for student in students {
        let classes: Vec<String> = super::get_classes(student.id)
            .await.into_iter()
                .filter(
                    |x| x.chars().last().unwrap() != '5'
                ).collect();

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

    (v_space, adj_list, student_map)
}