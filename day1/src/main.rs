use std::fs;
fn main() {

    let file_path = "puzzle.txt";

    let contents = fs::read_to_string(file_path).expect("boof");

    let mut list_of_words_from_file: Vec<&str> = Vec::new();

    for byte in contents.split_whitespace(){
        list_of_words_from_file.push(byte)
    }

    let output = sums_first_and_last_numeric_values_from_string(list_of_words_from_file);

    println!("{}", output)

}

fn sums_first_and_last_numeric_values_from_string(vector_of_strings: Vec<&str>) -> u32{

    // list of the words with their extracted numbers
    let mut numerics_per_word: Vec<Vec<u32>> = Vec::new();

    for item in vector_of_strings{
        // list to hold the numbers per word
        let mut nums_gatherer: Vec<u32> = Vec::new();
        for c in item.chars(){
            if c.is_numeric(){
                let num = c.to_digit(10).unwrap();
                nums_gatherer.push(num)
            }
        }
        numerics_per_word.push(nums_gatherer);
    }
    
    // this is here to make a complete sum of all the numeric values given from a word
    let mut gather_sums = 0;

    for num_array in numerics_per_word{

        let first_num = num_array[0];
        let last_num = num_array[num_array.len() -1];

        let first_char = first_num.to_string();
        let last_char = last_num.to_string();

        let total_string = first_char+&last_char;
        
        let my_int: u32 = total_string.parse().unwrap();

        gather_sums += my_int;
        
    }

    return gather_sums;

}