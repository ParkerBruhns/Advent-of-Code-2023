pub fn try_one() {
    let file: &str = r"src/input.txt";
    let input: Vec<String> = std::fs::read_to_string(file).unwrap()
        .lines().map(String::from).collect();

    let mut total: i32 = 0;

    for str in input.iter() {
        let mut num = String::new();
        let chars: Vec<char> = str.chars().collect();
        let mut buffer: String = String::new();
        // println!("{str}");

        for c in chars.iter() {
            buffer.push(*c);
            match find_int(&buffer) {
                Some(i) => {
                    num.push(char::from_digit(i as u32, 10).unwrap());
                    break;
                },
                None => {},
            };

            match c.to_string().parse::<i32>() {
                Err(_) => continue,
                Ok(x) => {
                    num.push(char::from_digit(x as u32, 10).unwrap());
                    break;
                }
            };
        }

        buffer = String::new();

        for c in chars.iter().rev() {
            buffer.push(*c);
            match find_int(&buffer.chars().rev().collect::<String>()) {
                Some(i) => {
                    num.push(char::from_digit(i as u32, 10).unwrap());
                    break;
                },
                None => {},
            };

            match c.to_string().parse::<i32>() {
                Err(_) => continue,
                Ok(x) => {
                    num.push(char::from_digit(x as u32, 10).unwrap());
                    break;
                }
            };
        }

        let out: &str = &num[..2];
        // println!("{out}");

        let len: usize = 2;
        let mut last: String = String::from(out);
        if out.len() < len {
            last = format!("{out}{out}");
        }

        // println!("{last}");
        total += last.parse::<i32>().unwrap();
    }
    println!("{total}");
}

fn str_to_int(buffer: &str) -> i32 {
    let str = buffer.to_lowercase();
    match str.as_str() {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        "zero" => 0,
        _ => panic!("NAN"),
    }
}

fn find_int(buffer: &String) -> Option<i32> {
    let numbers: [&str; 10] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "zero"];
    for num in numbers.iter() {
        if buffer.contains(num) {
            return Some(str_to_int(num));
        }
    }
    None
}
