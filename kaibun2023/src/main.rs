use std::collections::HashMap;

fn type_name<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

fn create_num_str() -> String {
    (1..=2023)
        .collect::<Vec<u32>>()
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join("")
}

fn find_palindrome(cs: &Vec<char>) -> HashMap<usize, Vec<usize>> {
    let mut result_map: HashMap<usize, Vec<usize>> = HashMap::new();

    for i in 0..cs.len() {
        // odd
        let mut j = 1;
        while i >= j && i + j < cs.len() {
            if cs[i - j] != cs[i + j] {
                break;
            }
            j += 1;
        }

        let mut cur_len = (j - 1) * 2 + 1;
        let mut cur_pos = i + 1 - j;
        if cur_len > 4 {
            // println!("cur_pos: {}", cur_pos);
            // println!("cur_len: {}", cur_len);
            // println!("{}", String::from_iter(&cs[cur_pos..cur_pos+cur_len]));

            match result_map.get_mut(&cur_len) {
                Some(vec) => vec.push(cur_pos),
                _ => {
                    let mut new_vec = Vec::<usize>::new();
                    new_vec.push(cur_pos);
                    result_map.insert(cur_len, new_vec);
                }
            }
        }

        // even
        j = 1;
        while i >= j && i + j < cs.len() {
            if cs[i - j] != cs[i + j - 1] {
                break;
            }
            j += 1;
        }

        cur_len = (j - 1) * 2;
        cur_pos = i + 1 - j;
        if cur_len > 4 {
            // println!("cur_pos: {}", cur_pos);
            // println!("cur_len: {}", cur_len);
            // println!("{}", String::from_iter(&cs[cur_pos..cur_pos+cur_len]));

            match result_map.get_mut(&cur_len) {
                Some(vec) => vec.push(cur_pos),
                _ => {
                    let mut new_vec = Vec::<usize>::new();
                    new_vec.push(cur_pos);
                    result_map.insert(cur_len, new_vec);
                }
            }
        }
    }
    // let mut new_vec = Vec::<usize>::new();
    // new_vec.push(4);
    // new_vec.push(5);
    // result_map.insert(3, new_vec);

    result_map
}

fn main() {
    let s = create_num_str();
    let cs = s.chars().collect::<Vec<char>>();
    let r = find_palindrome(&cs);
    let max_len = r
        .iter()
        .max_by(|a, b| a.0.cmp(&b.0))
        .map(|(k, _v)| k)
        .unwrap();

    let value = r.get(max_len).unwrap();
    // println!("{}", type_name(max_len));
    // println!("{}", type_name(value));
    for pos in value.iter() {
        let s = pos;
        let e = pos + max_len;
        println!("{}", String::from_iter(&cs[s..e]));
    }

    // for (key, value) in r.iter() {
    //     for pos in value.iter() {
    //         let s = pos;
    //         let e = pos + key;
    //         // println!("{}", s);
    //         // println!("{}", e);
    //         println!("{}", String::from_iter(&cs[*s..e]));
    //         // println!("{}", type_name(s));
    //         // println!("{}", type_name(e));
    //         // println!("{} {} {}", key, pos, String::from_iter(&cs[s..&e]));
    //     }
    // }
}
