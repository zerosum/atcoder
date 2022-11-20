//
// https://atcoder.jp/contests/abc042/tasks/abc042_b
//
fn main() {
    let mut xs: Vec<String> = Vec::new();
    
    let line1: Vec<i32> = read().trim().split(' ').map(|s| s.parse().unwrap()).collect();

    let mut cnt = 0;
    while cnt < line1[0] {
        xs.push(read());
        cnt += 1;
    };

    println!("{}", solver(xs))
}

fn read() -> String {
    let mut buf: String = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_owned()
}

fn solver(mut xs: Vec<String>) -> String {
    xs.sort();
    xs.concat()
}
