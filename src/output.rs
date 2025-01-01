use chrono::{DateTime, Local, Utc};
use terminal_size::{terminal_size, Height, Width};

pub fn get_terminal_size() -> (usize, usize) {
    let (Width(width), Height(height)) = terminal_size().expect("Failed to obtain a terminal size");
    (width as usize, height as usize)
}

pub fn format_date(date: &DateTime<Utc>) -> String {
    date.with_timezone(&Local)
        .format("%Y-%m-%d %H:%M:%S")
        .to_string()
}
