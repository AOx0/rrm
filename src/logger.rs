#[macro_export]
macro_rules! log {
    ( $na:tt : $($t:tt)* ) => {
        {
            use std::io::Write;
            use colored::*;
            let mut h = std::io::stdout();

            let label = stringify!($na);

            let label = if label == "Error" {
                label.red()
            } else if label == "Warning" {
                label.yellow()
            } else if label == "Received" {
                label.into()
            } else {
                label.green()
            }.bold();

            write!(h, "{:>12} ", label).unwrap();
            writeln!(h, $($t)* ).unwrap();
            h.flush().unwrap();
        }
    }
}
