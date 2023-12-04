/*
 * Author: Brandon Rhyno
 * Date: 12/03/2023
 * Description: Advent of Code day 3 in Rust
 *
 * --- Day 3: Gear Ratios ---
 * You and the Elf eventually reach a gondola lift station; he says the
 * gondola lift will take you up to the water source, but this is as far as he
 * can bring you. You go inside.
 *
 * It doesn't take long to find the gondolas, but there seems to be a problem:
 * they're not moving.
 *
 * "Aaah!"
 *
 * You turn around to see a slightly-greasy Elf with a wrench and a look of
 * surprise. "Sorry, I wasn't expecting anyone! The gondola lift isn't working
 * right now; it'll still be a while before I can fix it." You offer to help.
 *
 * The engineer explains that an engine part seems to be missing from the
 * engine, but nobody can figure out which one. If you can add up all the part
 * numbers in the engine schematic, it should be easy to work out which part
 * is missing.
 *
 * The engine schematic (your puzzle input) consists of a visual
 * representation of the engine. There are lots of numbers and symbols you
 * don't really understand, but apparently any number adjacent to a symbol,
 * even diagonally, is a "part number" and should be included in your sum.
 * (Periods (.) do not count as a symbol.)
 *
 * Here is an example engine schematic:
 * 467..114..
 * ...*......
 * ..35..633.
 * ......#...
 * 617*......
 * .....+.58.
 * ..592.....
 * ......755.
 * ...$.*....
 * .664.598..
 *
 * In this schematic, two numbers are not part numbers because they are not
 * adjacent to a symbol: 114 (top right) and 58 (middle right). Every other
 * number is adjacent to a symbol and so is a part number; their sum is 4361.
 *
 * Of course, the actual engine schematic is much larger. What is the sum of
 * all of the part numbers in the engine schematic?
 */

fn main() {
    // Read in the input file
    let input = std::fs::read_to_string("../Day3.txt").unwrap();

    //put the input into a 2d vector of chars
    let mut engine_schematic: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    //println!("{:?}", engine_schematic);
    let mut number = 0;
    let mut sum = 0;
    let mut valid = false;

    for i in 0..engine_schematic.len() {
        for j in 0..engine_schematic[i].len() {
            // If the current char is a number, check the adjacent chars
            if engine_schematic[i][j].is_digit(10) {
                number *= 10; // Shift the number over one place
                number += engine_schematic[i][j].to_digit(10).unwrap();
                if valid == false {
                    // check the char left
                    if j > 0 {
                        if engine_schematic[i][j - 1] != '.'
                            && engine_schematic[i][j - 1].is_digit(10) == false
                        {
                            valid = true;
                        }
                    }
                    // check the char right
                    if j < engine_schematic[i].len() - 1 {
                        if engine_schematic[i][j + 1] != '.'
                            && engine_schematic[i][j + 1].is_digit(10) == false
                        {
                            valid = true;
                        }
                    }
                    // check the char above
                    if i > 0 {
                        if engine_schematic[i - 1][j] != '.'
                            && engine_schematic[i - 1][j].is_digit(10) == false
                        {
                            valid = true;
                        }
                    }
                    // check the char below
                    if i < engine_schematic.len() - 1 {
                        if engine_schematic[i + 1][j] != '.'
                            && engine_schematic[i + 1][j].is_digit(10) == false
                        {
                            valid = true;
                        }
                    }
                    // Check the char above and to the left
                    if i > 0 && j > 0 {
                        if engine_schematic[i - 1][j - 1] != '.'
                            && engine_schematic[i - 1][j - 1].is_digit(10) == false
                        {
                            valid = true;
                        }
                    }
                    // Check the char above and to the right
                    if i > 0 && j < engine_schematic[i].len() - 1 {
                        if engine_schematic[i - 1][j + 1] != '.'
                            && engine_schematic[i - 1][j + 1].is_digit(10) == false
                        {
                            valid = true;
                        }
                    }
                    // Check the char below and to the left
                    if i < engine_schematic.len() - 1 && j > 0 {
                        if engine_schematic[i + 1][j - 1] != '.'
                            && engine_schematic[i + 1][j - 1].is_digit(10) == false
                        {
                            valid = true;
                        }
                    }
                    // Check the char below and to the right
                    if i < engine_schematic.len() - 1 && j < engine_schematic[i].len() - 1 {
                        if engine_schematic[i + 1][j + 1] != '.'
                            && engine_schematic[i + 1][j + 1].is_digit(10) == false
                        {
                            valid = true;
                        }
                    }
                }
            } else {
                if valid {
                    //println!("{} is a valid number", number);
                    sum += number;
                    valid = false;
                }
                number = 0;
            }
        }
    }
    println!("The sum of all the part numbers is: {}", sum);
}
