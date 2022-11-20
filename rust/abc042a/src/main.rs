// 
// https://atcoder.jp/contests/abc042/tasks/abc042_a
// 
fn main() {
    let mut x: String = String::new();
    std::io::stdin().read_line(&mut x).ok();
    println!("{}", solver(&x.trim()));
}

fn solver(str: &str) -> &str {
    let mut xs: Vec<&str> = str.split(' ').collect();
    xs.sort();
    let expected = vec!["5", "5", "7"];
    if xs == expected {
        "YES"
    } else {
        "NO"
    }
}

#[cfg(test)]
mod tests {
    use crate::solver;

    #[test]
    fn test() {
        assert_eq!(solver("5 5 7"), "YES");
        assert_eq!(solver("7 7 5"), "NO");
    }
}