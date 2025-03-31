/// Counts the number of command-line arguments passed to the program.
///
/// This function iterates over all the command-line arguments provided to the program
/// and returns their count as a `usize`. Note that the program name is also considered
/// an argument, so the count will always be at least one.
fn count_args() -> usize {
    let args = std::env::args();
    let mut count = 0;

    for _ in args {
        count += 1;
    }

    count
}

/// The main function of the program.
///
/// This function retrieves the count of command-line arguments using the `count_args()` function,
/// prints the count to the standard output, and returns an `io::Result<()>` indicating success or failure.
///
/// # Returns
///
/// * `Ok(())` - If the count is printed successfully.
/// * `Err(std::io::Error)` - If an error occurs during printing.
fn main() -> Result<(), std::io::Error> {
    let arg_count = count_args();
    println!("Number of arguments: {}", arg_count);

    Ok(())
}
