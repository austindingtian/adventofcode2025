pub mod day1 {
    pub mod part1;
    pub mod part2;
}

pub mod utils {
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;

    // file read lines implementation from Rust By Example
    // https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html#a-more-efficient-approach
    pub fn load_puzzle_input<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> 
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
}

pub use crate::utils::load_puzzle_input;
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
