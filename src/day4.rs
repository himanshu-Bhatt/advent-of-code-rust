use std::fs;

enum Part {
    One,
    Two,
}

fn read_file(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}

fn get_ans(input: &str, part: Part) -> u32 {
    let separators: &[char] = &[',', '-'];
    let count = input
        .lines()
        .filter(|line| {
            let arr: Vec<&str> = line.split(|c| separators.contains(&c)).collect();
            if let [Ok(a), Ok(b), Ok(c), Ok(d)] = [
                arr[0].parse::<i32>(),
                arr[1].parse::<i32>(),
                arr[2].parse::<i32>(),
                arr[3].parse::<i32>(),
            ] {
                match part {
                    Part::One => {
                        return b >= d && a <= c || d >= b && a >= c;
                    }
                    Part::Two => {
                        if b >= c && d >= a {
                            println!("{:?}", line);
                            return true;
                        }
                    }
                }
            }
            false
        })
        .count();

    count as u32
}

fn main() {
    if let Ok(res) = read_file("src/input/input4.txt") {
        let ans1 = get_ans(&res, Part::One);
        let ans2 = get_ans(&res, Part::Two);
        println!("{}", ans1);
        println!("{}", ans2);
    } else {
        println!("Error while opening file");
    }
}
