use std::env;

enum Filesize{
    Byte(u64),
    Kilobyte(f64),
    Megabyte(f64),
    Gigabyte(f64),
}

fn format_size(size:usize) -> String{
    let filesize = match size{
        0..=999 => Filesize::Byte(size as u64),
        1000..=999999 => Filesize::Kilobyte(size as f64/1000.0),
        1_000_000..=999_999_999 => Filesize::Megabyte(size as f64/1_000_000.0),
        1_000_000_000..=999_999_999_999 => Filesize::Gigabyte(size as f64/1_000_000_000.0),
        _ => Filesize::Gigabyte(size as f64 / 1_000_000_000.0),

    };

    match filesize {
        Filesize::Byte(byte) => format!("{}B",byte),
        Filesize::Kilobyte(kb) => format!("{:.2}KB",kb),
        Filesize::Megabyte(mb) => format!("{:.2}MB",mb),
        Filesize::Gigabyte(gb) => format!("{:.2}KB",gb),
    }
}

fn main() {

    let mut args:Vec<String> = env::args().collect();
    let inp_size = &args[1];
    println!("{}", inp_size);

    let result = format_size(9824576);
    println!("{}", result);
}