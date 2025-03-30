use std::io::{self, Write};

pub fn rust_putchar(c: char) -> io::Result<()> {
    let mut stdout = io::stdout();

    write!(&mut stdout, "{}", c)?;
    stdout.flush()?;

    Ok(())
}

/*
fn main() -> io::Result<()> {
    rust_putchar('1')?;
    rust_putchar('\n')?;
    Ok(())
}
*/
