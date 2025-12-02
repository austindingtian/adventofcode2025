use adventofcode2025::utils;

fn main() {
    if let Ok(lines) = utils::load_puzzle_input("./day1/input.txt") {
        //Consume the file iterator
        for line in lines.map_while(Result::ok) {
            println!("{}", line);
        }
    }
}
