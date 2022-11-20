fn main() {
    let l: Vec<String> = read().split(' ').map(|s| s.to_owned()).collect();
    let mut avoid_nums: Vec<char> = read().replace(' ', "").chars().collect();
    avoid_nums.sort();

    let result = solver(l[0].chars().collect(), avoid_nums);
    result.iter().for_each(|c| print!("{}", c));
    print!("\n")
}

fn read() -> String {
    let mut buf: String = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_owned()
}

fn solver(price_str: Vec<char>, avoid_nums: Vec<char>) -> Vec<char> {
    let nums: Vec<char> = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']
        .iter()
        .filter(|c| !avoid_nums.contains(c))
        .map(|c| *c)
        .collect();
    let minimum: char = nums[0];

    let mut candidate_larger: Vec<char> = Vec::new();
    let mut candidate_smaller: Vec<char> = Vec::new();
    let mut cnt: usize = 0;
    let price_str_len = price_str.len();
    while cnt < price_str_len {
        let d = price_str[cnt];
        let ge: Vec<&char> = nums.iter().filter(|n| d <= **n).collect();

        match (ge.get(0), ge.get(1)) {
            (None, _) => {
                while cnt < price_str_len {
                    candidate_larger.push(minimum);
                    cnt += 1;
                }
                break
            }
            (Some(i1), _) if **i1 > d => {
                candidate_smaller.push(**i1);
                while cnt < price_str_len - 1 {
                    candidate_smaller.push(minimum);
                    cnt += 1;
                }
                break
            }
            (Some(i1), Some(i2)) => {
                candidate_larger = candidate_smaller.clone();
                candidate_larger.push(**i2);
                candidate_smaller.push(**i1)
            }
            (Some(i1), None) => {
                candidate_larger.push(minimum);
                candidate_smaller.push(**i1)
            }
        }
        cnt += 1
    }

    if candidate_smaller >= price_str {
        candidate_smaller
    } else if candidate_larger >= price_str {
        candidate_larger
    } else {
        let mut carried: Vec<char> = {
            if minimum == '0' {
                vec![nums[1]]
            } else {
                vec![minimum]
            }
        };
        carried.append(&mut candidate_larger);
        carried
    }
}
