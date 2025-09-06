fn main() {
    let a: i32 = 49;

    let pi: f64 = 3.14159;

    let s1 = String::from("hello");

    let mut ans = (a as f64) * pi;
    ans += 1.10199;

    println!("This is checking for mutable {}", 156.0 - ans);

    println!("{}", s1);
    println!("{}", (a as f64) * pi);
    println!("Datatypes....");
}
