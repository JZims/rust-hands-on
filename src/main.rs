use rand::Rng;
// use std::io::stdin;

fn roll_dice(sides: u32, number: u32, modifier: Option<i32>) -> (i32, String) {
    let mut rng = rand::thread_rng(); // Secure random number generator.
    let mut total = 0;
    let mut details = Vec::new();

    // Roll the dice the specified number of times.
    for _ in 0..number {
        let roll = rng.gen_range(1..=sides) as i32; // Generate a random number between 1 and `sides`.
        total += roll;
        details.push(roll.to_string());
    }

    // Apply the modifier, if there is one.
    let final_total = match modifier {
        Some(mod_value) => total + mod_value,
        None => total,
    };

    // Create a summary of the rolls and the modifier.
    let summary = format!(
        "Rolled {}d{}: [{}] {} Modifier: {}",
        number,
        sides,
        details.join(", "),
        if modifier.is_some() { "+" } else { "" },
        modifier.unwrap_or(0)
    );

    (final_total, summary)
}

// fn determine_dice() -> Vec<i32>{
//     let mut results:Vec<i32> = Vec::new();
//     let mut new_value: i32 = Number::new();
//     stdin()
//     .read_line()

// }

fn main() {

    let (result, summary) = roll_dice(6, 3, Some(2));
    println!("Result: {}", result);
    println!("Summary: {}", summary);
}






// fn what_is_your_name() -> String {
//     let mut your_name = String::new();
//     stdin()
//     .read_line(&mut your_name)
//     .expect("Failed to read line");

//     your_name
//     .trim()
//     .to_lowercase()
// }

// fn main() {
//     println!("Hello, what is your name?");
//     let name = what_is_your_name();
//     println!("Hello, {}!", name );
// }