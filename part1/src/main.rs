fn main() {
    // println!("Hello, world!");
    let ans: bool = is_even(17);
    println!("{}", ans)
}

fn is_even(num: i32) -> bool{
    if num % 2 == 0 {
        return true;
    }
    return false;
}
