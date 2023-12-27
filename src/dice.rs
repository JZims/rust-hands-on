
// use rand::Rng; // Ra;nd crate is used for generating random numbers.
// /// Rolls a specified number of dice, each with a specified number of sides, and applies an optional modifier.
// ///
// /// # Arguments
// ///
// /// * `sides` - A positive integer specifying the number of sides on the dice.
// /// * `number` - A positive integer specifying how many dice to roll.
// /// * `modifier` - An optional integer that modifies the final result.
// ///
// /// # Returns
// ///
// /// A tuple containing the final result and a summary of the rolls.
// fn roll_dice(sides: u32, number: u32, modifier: Option<i32>) -> (i32, String) {
//     let mut rng = rand::thread_rng(); // Secure random number generator.
//     let mut total = 0;
//     let mut details = Vec::new();

//     // Roll the dice the specified number of times.
//     for _ in 0..number {
//         let roll = rng.gen_range(1..=sides) as i32; // Generate a random number between 1 and `sides`.
//         total += roll;
//         details.push(roll.to_string());
//     }

//     // Apply the modifier, if there is one.
//     let final_total = match modifier {
//         Some(mod_value) => total + mod_value,
//         None => total,
//     };

//     // Create a summary of the rolls and the modifier.
//     let summary = format!(
//         "Rolled {}d{}: [{}] {} Modifier: {}",
//         number,
//         sides,
//         details.join(", "),
//         if modifier.is_some() { "+" } else { "" },
//         modifier.unwrap_or(0)
//     );

//     (final_total, summary)
// }

// fn main() {
//     // Example usage:
//     let (result, summary) = roll_dice(6, 3, Some(-2));
//     println!("Result: {}", result);
//     println!("Summary: {}", summary);
// }

