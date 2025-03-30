// https://doc.rust-lang.org/std/macro.write.html

use std::io::{self, Write};

pub fn rust_putchar(c: char) -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    write!(handle, "{}", c)?;
    handle.flush()?;

    Ok(())
}

// // test
// fn main() -> io::Result<()> {
//     rust_putchar('A')?;
//     rust_putchar('\n')?;
//     Ok(())
// }

