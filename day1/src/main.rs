use std::{fs::File, io::Read};

fn main() {
    let mut file: File = File::open("./src/input.txt").unwrap();
    let mut records = String::new();
    file.read_to_string(&mut records).unwrap();
    let records = records.split("\n");

    let mut elves = vec![];
    let mut local_sum: i32 = 0;

    for record in records {
        match record {
            "" => {
                // println!("{}", local_sum);
                elves.push(local_sum);
                local_sum = 0
            },
            _ => {
                local_sum += record.parse::<i32>().unwrap();
            },
        }
    }
    elves.sort();
    elves.reverse();
    let mut top_three_sum = 0;
    for i in 0..3 {
        top_three_sum += elves[i];
    }
    println!("{}", top_three_sum)
}
