use std::{
    cmp,
    io::{self, BufRead},
};

fn main() {
    let (correct_words, incorrect_words) = handle_input();
    for incorrect_word in &incorrect_words {
        let mut inner_distance: Vec<(usize, String)> = Vec::new();
        let mut prev_min_d = u32::MAX as usize;
        for correct_word in &correct_words {
            let out = distance(incorrect_word, correct_word, prev_min_d);
            if out < prev_min_d {
                prev_min_d = out;
            }
            inner_distance.push((out, correct_word.to_owned()));
        }
        let min = inner_distance.iter().min().unwrap().0;
        let smallest_distance: Vec<&(usize, String)> =
            inner_distance.iter().filter(|x| x.0 == min).collect();
        print!("{} ({})", incorrect_word, smallest_distance[0].0);
        for word in smallest_distance {
            print!(" {}", word.1);
        }
        print!("\n");
    }
}

fn distance(sa: &str, sb: &str, prev_min_d: usize) -> usize {
    if sa == sb {
        return 0;
    }

    if sa.len() == 0 {
        return sb.len();
    }
    if sb.len() == 0 {
        return sa.len();
    }

    let chars_a: Vec<char> = sa.chars().collect();
    let chars_b: Vec<char> = sb.chars().collect();
    let mut arr: Vec<Vec<usize>> = vec![vec![0; chars_a.len() + 1]; chars_b.len() + 1];
    for x in 0..=chars_b.len() {
        arr[x][0] = x;
    }
    for y in 0..=chars_a.len() {
        arr[0][y] = y;
    }

    for x in 1..=(chars_b.len()) {
        for y in 1..=(chars_a.len()) {
            if chars_b[x - 1] == chars_a[y - 1] {
                arr[x][y] = arr[x - 1][y - 1]
            } else {
                arr[x][y] = cmp::min(cmp::min(arr[x - 1][y - 1], arr[x][y - 1]), arr[x - 1][y]) + 1;
            }
        }
        if x < chars_a.len() {
            let mut fi = 0;
            let mut si = 0;
            if chars_a.len() < chars_b.len() {
                fi = chars_b.len() - chars_a.len();
            } else {
                si = chars_a.len() - chars_b.len();
            }
            if arr[fi + x][si + x] > prev_min_d {
                return u32::MAX as usize;
            }
        }
    }

    // error checking
    // for x in &arr {
    //     eprintln!("{:?}", x)
    // }
    arr[chars_b.len()][chars_a.len()]
}

fn handle_input() -> (Vec<String>, Vec<String>) {
    let input = io::stdin();
    let mut lines = input.lock().lines();

    let mut correct_words_vec: Vec<String> = Vec::new();
    let mut incorrect_words_vec: Vec<String> = Vec::new();
    let mut correct_words = true;
    while let Some(line) = lines.next() {
        let input = match line {
            Ok(string) => string,
            Err(_) => {
                continue;
            }
        };
        if input.len() == 0 {
            break;
        }
        input.trim();
        if correct_words {
            if input.is_empty() == true {
                continue;
            }
            if input == "#" {
                correct_words = false;
                continue;
            }

            correct_words_vec.push(input);
            continue;
        }

        incorrect_words_vec.push(input);
    }

    return (correct_words_vec, incorrect_words_vec);
}
