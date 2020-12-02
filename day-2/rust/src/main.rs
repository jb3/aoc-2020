
#[derive(Debug)]
struct PasswordPolicy {
    leftmost: usize,
    rightmost: usize,
    character: char
}

impl PasswordPolicy {
    pub fn from_policy(line: &str) -> Self {
        // Split the policy ("12-14 a") into a range and a character
        let mut splitted = line.split(' ');

        // Unwrap the values
        let range = splitted.next().unwrap();
        let character_str = splitted.next().unwrap();

        // Split the range into two integers
        let mut split_res = range.split('-');

        // Parse the integers in string form into usize
        let leftmost: usize = split_res.next().unwrap().parse().unwrap();
        let rightmost: usize = split_res.next().unwrap().parse().unwrap();

        // Convert the character into a char from an &str
        let character = character_str.chars().next().unwrap();

        // Create a new policy struct
        Self {
            leftmost,
            rightmost,
            character
        }
    }

    fn meets_repetition_policy(&self, password: String) -> bool {
        let appearances = password
            // Fetch the characters in the password
            .chars()
            // Filter to only those that match the policy character
            .filter(|&c| c == self.character)
            // Count the items
            .count();

        // Check the items are within the specified ranges in the policy
        self.leftmost <= appearances && appearances <= self.rightmost
    }

    fn meets_position_policy(&self, password: String) -> bool {
        // Get an iterator of characters
        let mut chars = password.chars();

        // Fetch the leftmost character (1-indexed)
        let leftmost = chars.nth(self.leftmost - 1).unwrap() == self.character;
        // Fetch the rightmost character (the iterator starts at the position of the leftmost character)
        let rightmost = chars.nth(self.rightmost - self.leftmost - 1).unwrap() == self.character;

        // XOR the booleans, only one position can hold the character
        leftmost ^ rightmost
    }
}

fn get_input() -> Vec<(PasswordPolicy, String)> {
    // Create a string from the inlined input file.
    let input_string = String::from(include_str!("../../input.txt"));

    input_string
        // Split into lines
        .lines()
        .map(|line| {
            // Split the line into the policy and the password
            let mut splitted = line.split(": ");
            
            // Unwrap the values
            let policy_str = splitted.next().unwrap();
            let password = splitted.next().unwrap();

            // Create a password policy struct
            (PasswordPolicy::from_policy(policy_str), password)
        })
        // Convert the &str into Strings
        .map(|(policy, pw)| (policy, String::from(pw)))
        .collect()
}

fn main() {
    let input = get_input();

    // Find the puzzle inputs
    let valid_p1 = input
        // Create an iterator
        .iter()
        // Filter to the matching repetition passwords
        .filter(|(policy, password)| policy.meets_repetition_policy(password.to_string()))
        // Find the total valid passwords
        .count();

    println!("Part 1: {}", valid_p1);

    let valid_p2 = input
        // Create an iterator
        .iter()
        // Filter to the matching position passwords
        .filter(|(policy, password)| policy.meets_position_policy(password.to_string()))
        // Find the total valid passwords
        .count();

    println!("Part 2: {}", valid_p2);
}
