use std::io::{self, Write};

/// Writes a single character to the standard output (stdout).
///
/// This function locks the stdout for exclusive access during the write operation,
/// writes the given character to the terminal, and then flushes the output to ensure
/// immediate display. It returns an `io::Result<()>`, indicating success or failure.
///
fn rust_putchar(c: char) -> Result<(), std::io::Error> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    write!(handle, "{}", c)?;
    handle.flush()?;

    Ok(())
}

/// The main function of the program.
///
/// This function prints all lowercase English letters from 'a' to 'z',
/// followed by a newline character.
///
/// # Returns
///
/// * `Ok(())` if all characters are successfully printed.
/// * `Err(std::io::Error)` if an I/O error occurs during the printing process.
fn main() -> Result<(), std::io::Error> {
    for letter in 'a'..='z' {
        let _ = rust_putchar(letter);
    }
    let _ = rust_putchar('\n');

    Ok(())
}
