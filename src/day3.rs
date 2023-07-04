use std::collections::HashSet;

fn sum(input: String) -> u32 {
    let mut sum = 0;
    let mut set = std::collections::HashSet::new();
    for line in input.lines() {
        let length = line.chars().count();
        let mid = length / 2;
        for i in 0..mid {
            set.insert(line.chars().nth(i).unwrap());
        }
        for i in mid..length {
            let c = line.chars().nth(i).unwrap();
            if set.contains(&c) {
                if c as u32 <= 'Z' as u32 {
                    sum += c as u32 - 'A' as u32 + 27;
                } else {
                    sum += c as u32 - 'a' as u32 + 1;
                }
                break;
            }
        }
        set.clear();
    }
    return sum;
}
fn find_common(lines: &Vec<&str>) -> Option<u32> {
    let mut common_chars: Option<HashSet<char>> = None;
    for line in lines {
        let lines_chars: HashSet<char> = line.chars().collect();
        if let Some(chars) = common_chars {
            common_chars = Some(chars.intersection(&lines_chars).cloned().collect());
        } else {
            common_chars = Some(lines_chars);
        }
    }
    if let Some(chr) = common_chars {
        if let Some(ch) = chr.iter().next() {
            if *ch as u32 <= 'Z' as u32 {
                return Some(*ch as u32 - 'A' as u32 + 27);
            } else {
                return Some(*ch as u32 - 'a' as u32 + 1);
            }
        }
    }

    return None;
}
fn sumGroup(input: String) -> u32 {
    let mut sum = 0;
    let chunks = input.lines().chunks(3);
    for chunk in &chunks {
        let lines: Vec<&str> = chunk.collect();
        if let Some(num) = find_common(&lines) {
            sum += num;
        }
    }

    return sum;
}

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let ans = sumGroup(input);
    println!("Sum is : {}", ans);
}
