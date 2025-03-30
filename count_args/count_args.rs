use std::env;

/// Counts the number of command-line arguments passed to the program.
///
/// This function iterates over all the command-line arguments provided to the program
/// and returns their count as a `usize`. Note that the program name is also considered
/// an argument, so the count will always be at least one.
fn count_args() -> usize {
    let args = env::args();
    let mut count = 0;

    for _ in args {
        count += 1;
    }

    count
}

fn main() {
    let arg_count = count_args();
    println!("Number of arguments: {}", arg_count);
}
