/*
 * Author: Brandon Rhyno
 * Date: 12/01/2023
 * Description: Advent of Code day 1 in Rust
 *
 * --- Part Two ---
 * Your calculation isn't quite right. It looks like some of the digits are
 * actually spelled out with letters: one, two, three, four, five, six, seven,
 * eight, and nine also count as valid "digits".
 *
 * Equipped with this new information, you now need to find the real first and
 * last digit on each line. For example:
 * two1nine
 * eightwothree
 * abcone2threexyz
 * xtwone3four
 * 4nineeightseven2
 * zoneight234
 * 7pqrstsixteen
 *
 * In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76.
 * Adding these together produces 281.
 *
 * What is the sum of all of the calibration values?
 */
fn main() {
    //read in the file
    let input = std::fs::read_to_string("../Day1.txt").unwrap();
    let mut sum = 0;
    let number_key = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let spelt_number_key = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    //iterate through each line
    for line in input.lines() {
        let mut lowest_index = line.len();
        let mut highest_index = 0;

        //Replace any spelled out numbers with their numerical equivalent
        let mut all_number_line = line.to_string();
        for i in 0..spelt_number_key.len() {
            let first_char = spelt_number_key[i]
                .chars().nth(0).unwrap();
            let last_char = spelt_number_key[i]
                .chars()
                .nth(spelt_number_key[i].len() - 1)
                .unwrap();
            let replacement = format!("{}{}{}", first_char, &number_key[i].to_string(), last_char); // Keep the first and last character, but replace the middle with the number

            all_number_line = all_number_line.replace(spelt_number_key[i], &replacement);
        }

        for (i, character) in all_number_line.chars().enumerate() {
            for number in number_key.iter() {
                if character == *number {
                    if i < lowest_index {
                        lowest_index = i;
                    }
                    if i > highest_index {
                        highest_index = i;
                    }
                }
            }
        }
        println!("{}", all_number_line);
        //add the first and last number together
        let first_number = all_number_line.chars().nth(lowest_index).unwrap();
        let last_number = all_number_line.chars().nth(highest_index).unwrap();

        //combine the two numbers into a single number
        let line_number =
            first_number.to_digit(10).unwrap() * 10 + last_number.to_digit(10).unwrap();
        //println!("{}", line_number);
        sum += line_number;
    }
    println!("The sum of all the calibration values is {}", sum);
}
