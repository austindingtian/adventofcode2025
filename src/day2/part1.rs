use crate::load_puzzle_input;

pub fn solution () {
    let mut id_ranges: Vec<IDRange> = Vec::new();

    // read in product id ranges
    if let Ok(input) = load_puzzle_input("./src/day2/input.txt") {
        for line in input.map_while(Result::ok) {
            let ranges = line.split(',');
            for range in ranges {
                id_ranges.push(IDRange::split_range(range));
            }
        }
    }

    // process id ranges to find invalid ids
    let mut invalid_ids: Vec<usize> = Vec::new();
    let mut invalid_sum = 0;
    for id_range in id_ranges {
        let (invalids_in_range, isum) = process_id_range(id_range);
        invalid_ids.extend(invalids_in_range);
        invalid_sum += isum;
    }
    println!("The invalid sum is, {}", invalid_sum);
}

struct IDRange(usize, usize);

impl IDRange {
    // range has to be a string in the form of `id-id`
    // that can be split by the hyphen and parsed into 2 integers
    fn split_range(range: &str) -> IDRange {
        let split: Vec<&str> = range.split('-').collect();
        IDRange(split[0].parse::<usize>().unwrap(), split[1].parse::<usize>().unwrap())
    }
}

// Iterates through the range of IDs and returns the list of invalid ones
fn process_id_range(id_range: IDRange) -> (Vec<usize>, usize) {
    let mut invalid_ids = Vec::new();
    let mut invalid_sum = 0;

    let lower = id_range.0;
    let upper = id_range.1;

    for id in lower..upper+1 {
        if !id_is_valid(id) {
            invalid_ids.push(id);
            invalid_sum += id;
        }
    }
    (invalid_ids, invalid_sum)
}

// Counts the number of digits in a given unsigned integer
pub fn count_digits(n: usize) -> usize {
    let mut temp = n;
    let mut count = 0;

    while temp != 0 {
        temp /= 10;
        count += 1;
    }
    count
}

// Check if a given ID is invalid or not based on the repeated sequence rule given by the puzzle
fn id_is_valid(id: usize) -> bool {
    let id_length = count_digits(id);
    
    if id_length % 2 != 0 {
        return true
    }
    
    let mut temp = id;
    let mut half = Vec::with_capacity(id_length/2);
    for _ in 0..id_length/2 {
        half.push(temp % 10);
        temp /= 10;
    }
    
    for i in 0..id_length/2 {
        if temp % 10 != half[i] { return true }
        temp /= 10;
    }
    
    return false
}

// separates the digits in a number into a list
// pub fn separate_digits(n: usize) -> Vec<usize> {
//     let mut temp = n;
//     let mut digits = Vec::new();

//     while temp != 0 {
//         digits.push(temp % 10);
//         temp /= 10;
//     }
//     digits
// }