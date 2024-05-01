fn main() {
    for (i, arg) in std::env::args().enumerate() {
        if i > 0 {
            print!(" ")
        }
        print!("{}", arg)
    }
    print!("\n")
}
