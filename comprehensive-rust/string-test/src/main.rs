fn main() {
    let s1: &str = "world";
    let s2: String = String::from("hello ");
    let s3: String = s2 + "s1"; // note s2 has been moved here and can no longer be used
    s2.push_str(s1);
    println!("{}", s2);
    println!("{}", s3);
}
