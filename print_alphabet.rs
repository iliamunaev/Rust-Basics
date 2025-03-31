use std::io::{self, Write};

/// Writes a single character to the standard output (stdout).
///
/// This function locks the stdout for exclusive access during the write operation,
/// writes the given character to the terminal, and then flushes the output to ensure
/// immediate display. It returns an `io::Result<()>`, indicating success or failure.
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
fn main() -> Result<(), std::io::Error> {
    for letter in 'a'..='z' {
        rust_putchar(letter)?;
    }
    rust_putchar('\n')?;

    Ok(())
}
