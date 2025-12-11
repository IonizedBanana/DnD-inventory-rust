use std::io;

fn main() {
    const OPTIONS: &str = "1. view inventory\n2. view weapons\n3. view notebook";

    loop {
        println!("please select and option:\n {OPTIONS}");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("error reading input!");
        let input: i32 = input.trim().parse().expect("could not parse!");
        println!("you input {input}")
    }
}
