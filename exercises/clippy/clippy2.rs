// clippy2.rs
// Make me compile! Execute `rustlings hint clippy2` for hints :)


fn main() {
    let mut res = 42;
    let opt = Some(12);
    if let Some(x) = opt {
        res = x;
    }
    println!("{}", res);
}
