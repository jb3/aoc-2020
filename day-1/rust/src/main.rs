const SUM_GOAL: i64 = 2020;

fn get_input() -> Vec<i64> {
    // Create a string from the inlined input file.
    let input_string = String::from(include_str!("../../input.txt"));

    // Parse the string to a list of integers
    input_string
        // Split the string by lines
        .lines()
        // Parse each line to an integer
        .map(|i| {
            i.parse::<i64>()
                .expect(&format!("Could not parse line: {}", i))
        })
        // Collect these mapped values into a vector
        .collect()
}

fn main() {
    // Fetch our input
    let input = get_input();

    // =-=- PART ONE -=-=

    // Define a tuple to store our answers in
    let mut combo_one = (0, 0);

    // Use two loops to generate the combinations
    'part_one: for a in &input {
        for b in &input {
            // If the two combinations sum to SUM_GOAL
            if a + b == SUM_GOAL {
                // Set the answer tuple
                combo_one = (a.clone(), b.clone());
                // Break the outer loop
                break 'part_one;
            }
        }
    }

    let part_one = combo_one.0 * combo_one.1;
    println!("Part 1: {}", part_one);

    // =-=- PART TWO -=-=

    // Create our answer tuple to store the combinations
    let mut combo_two = (0, 0, 0);

    // Like part one, define loops to generate our combinations.
    'part_two: for a in &input {
        for b in &input {
            for c in &input {
                // If we meet our sum goal
                if a + b + c == SUM_GOAL {
                    // Set the answer tuple
                    combo_two = (a.clone(), b.clone(), c.clone());
                    // Halt the outermost loop
                    break 'part_two;
                }
            }
        }
    }

    // Calculate the part 2 answer
    let part_two = combo_two.0 * combo_two.1 * combo_two.2;
    println!("Part 2: {}", part_two);
}
