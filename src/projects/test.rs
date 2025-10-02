//This is a test file


fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let res  = add(1, 2).to_string();
    println!("{}", res);
}
