use std::io::Read;

const ITEMS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

enum Item<'a> {
    Lowercase(&'a char),
    Uppercase(&'a char)
}

fn get_item_property(item: char) -> i32 {
    let find_result = ITEMS.find(item);
    return match find_result {
        Some(index) => 1 + index as i32,
        None => panic!("Letter {} not in alphabet.", item)
    }
}

fn find_main_item(data: &str) -> Result<char, String> {
    let part_len = data.len() / 2;
    let (first_part, second_part) = data.split_at(part_len);

    for item in first_part.chars() {
        let find_result = second_part.find(item);
        match find_result {
            Some(_) => return Ok(item),
            None => continue
        }
    }
    return Err(format!("No matching items found in rucksack: {}", data));
}

fn main() {
    let mut file_content = String::new();
    let mut file = std::fs::File::open("input.txt").unwrap();
    file.read_to_string(&mut file_content).unwrap();
    
    let rucksacks = file_content.lines();
    let mut properties_sum = 0;

    for rucksack in rucksacks {
        let result = find_main_item(rucksack);
        match result {
            Ok(item) => properties_sum += get_item_property(item),
            Err(e) => panic!("{}", e)
        }
    }
    println!("{}", properties_sum);
}
