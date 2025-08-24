fn main() {
    // println!("Hello, world!");
    // let ans: bool = is_even(18);
    let ans: i32 = fib(6);
    println!("{}", ans)
}

// fn is_even(num: i32) -> bool{
//     if num % 2 == 0 {
//         return true;
//     }
//     return false;
// }

fn fib(num: i32) -> i32 {
    if num < 2{
        return num;
    }
    // println!("{}", num);
    return fib(num-1) + fib(num-2);
}
