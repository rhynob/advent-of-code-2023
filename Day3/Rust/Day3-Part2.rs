/*
 * Author: Brandon Rhyno
 * Date: 12/03/2023
 * Description: Advent of Code day 3 in Rust
 *
 * --- Part Two ---
 * The engineer finds the missing part and installs it in the engine! As the
 * engine springs to life, you jump in the closest gondola, finally ready to
 * ascend to the water source.
 *
 * You don't seem to be going very fast, though. Maybe something is still
 * wrong? Fortunately, the gondola has a phone labeled "help", so you pick it
 * up and the engineer answers.
 *
 * Before you can explain the situation, she suggests that you look out the
 * window. There stands the engineer, holding a phone in one hand and waving
 * with the other. You're going so slowly that you haven't even left the
 * station. You exit the gondola.
 *
 * The missing part wasn't the only issue - one of the gears in the engine is
 * wrong. A gear is any * symbol that is adjacent to exactly two part numbers.
 * Its gear ratio is the result of multiplying those two numbers together.
 *
 * This time, you need to find the gear ratio of every gear and add them all
 * up so that the engineer can figure out which gear needs to be replaced.
 *
 * Consider the same engine schematic again:
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
 * In this schematic, there are two gears. The first is in the top left; it
 * has part numbers 467 and 35, so its gear ratio is 16345. The second gear is
 * in the lower right; its gear ratio is 451490. (The * adjacent to 617 is not
 * a gear because it is only adjacent to one part number.) Adding up all of
 * the gear ratios produces 467835.
 *
 * What is the sum of all of the gear ratios in your engine schematic?
 */

fn main() {
    // Read in the input file
    let input = std::fs::read_to_string("../Day3.txt").unwrap();

    //put the input into a 2d vector of chars
    let mut engine_schematic: Vec<Vec<char>> =
        input.lines().map(|line| line.chars().collect()).collect();
    //println!("{:?}", engine_schematic);
    let mut cog: Vec<Vec<u32>> = Vec::new();
    let mut sum = 0;

    for i in 0..engine_schematic.len() {
        for j in 0..engine_schematic[i].len() {
            if engine_schematic[i][j] == '*' {
                //check all around the cog for numbers

                cog.push(find_cog_value("up", i, j, &engine_schematic, Vec::new()));
                cog.push(find_cog_value("down", i, j, &engine_schematic, Vec::new()));
                cog.push(find_cog_value("left", i, j, &engine_schematic, Vec::new()));
                cog.push(find_cog_value("right", i, j, &engine_schematic, Vec::new()));

                if cog[0].len() == 0 {
                    cog.push(find_cog_value("upper_left", i, j, &engine_schematic, Vec::new()));
                    cog.push(find_cog_value("upper_right", i, j, &engine_schematic, Vec::new()));
                }
                if cog[1].len() == 0 {
                    cog.push(find_cog_value("lower_left", i, j, &engine_schematic, Vec::new()));
                    cog.push(find_cog_value("lower_right", i, j, &engine_schematic, Vec::new()));
                }

                let mut product = 1;
                // remove duplicates and empty vectors
                cog.sort();
                cog.dedup();
                cog.retain(|x| x.len() > 0);

                println!("{:?}", cog);
                for number in cog.iter() {
                    if cog.len() == 2
                        {
                            product *= number.iter().fold(0, |acc, x| acc * 10 + x);
                        }
                        else {
                            product = 0;
                        }
                }
                sum += product;
                println!("The product of the cog is: {}", product);
                cog.clear();
            }
        }
    }
    println!("The sum of all the part numbers is: {}", sum);
}

fn find_cog_value(
    direction: &str,
    i: usize,
    j: usize,
    engine_schematic: &Vec<Vec<char>>,
    mut number: Vec<u32>,
) -> Vec<u32> {
    // Check up
    if direction == "up" && i > 0 {
        if engine_schematic[i - 1][j].is_digit(10) {
            number.push(engine_schematic[i - 1][j].to_digit(10).unwrap());
            number = find_cog_value("left", i - 1, j, engine_schematic, number);
            number = find_cog_value("right", i - 1, j, engine_schematic, number);
        }
    }
    // Check down
    if direction == "down" && i < engine_schematic.len() - 1 {
        if engine_schematic[i + 1][j].is_digit(10) {
            number.push(engine_schematic[i + 1][j].to_digit(10).unwrap());
            number = find_cog_value("left", i + 1, j, engine_schematic, number);
            number = find_cog_value("right", i + 1, j, engine_schematic, number);
        }
    }
    // Check left
    if direction == "left" && j > 0 {
        if engine_schematic[i][j - 1].is_digit(10) {
            number.insert(0, engine_schematic[i][j - 1].to_digit(10).unwrap());
            number = find_cog_value("left", i, j - 1, engine_schematic, number);
        }
    }
    // Check right
    if direction == "right" && j < engine_schematic[i].len() - 1 {
        if engine_schematic[i][j + 1].is_digit(10) {
            number.push(engine_schematic[i][j + 1].to_digit(10).unwrap());
            number = find_cog_value("right", i, j + 1, engine_schematic, number);
        }
    }
    // Check upper left
    if direction == "upper_left" && i > 0 && j > 0 {
        if engine_schematic[i - 1][j - 1].is_digit(10) {
            number.push(engine_schematic[i - 1][j - 1].to_digit(10).unwrap());
            number = find_cog_value("left", i - 1, j - 1, engine_schematic, number);
            number = find_cog_value("right", i - 1, j - 1, engine_schematic, number);
        }
    }
    // Check upper right
    if direction == "upper_right" && i > 0 && j < engine_schematic[i].len() - 1 {
        if engine_schematic[i - 1][j + 1].is_digit(10) {
            number.push(engine_schematic[i - 1][j + 1].to_digit(10).unwrap());
            number = find_cog_value("left", i - 1, j + 1, engine_schematic, number);
            number = find_cog_value("right", i - 1, j + 1, engine_schematic, number);
        }
    }
    // Check lower left
    if direction == "lower_left" && i < engine_schematic.len() - 1 && j > 0 {
        if engine_schematic[i + 1][j - 1].is_digit(10) {
            number.push(engine_schematic[i + 1][j - 1].to_digit(10).unwrap());
            number = find_cog_value("left", i + 1, j - 1, engine_schematic, number);
            number = find_cog_value("right", i + 1, j - 1, engine_schematic, number);
        }
    }
    // Check lower right
    if direction == "lower_right"
        && i < engine_schematic.len() - 1
        && j < engine_schematic[i].len() - 1
    {
        if engine_schematic[i + 1][j + 1].is_digit(10) {
            number.push(engine_schematic[i + 1][j + 1].to_digit(10).unwrap());
            number = find_cog_value("left", i + 1, j + 1, engine_schematic, number);
            number = find_cog_value("right", i + 1, j + 1, engine_schematic, number);
        }
    }

    // Return the modified number vector
    number
}
