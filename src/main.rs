use std::{fs::File, io::{Read}};

fn main() -> std::io::Result<()> {
    let mut data = String::new();
    let mut file = File::open("sample.mbox")?;
    let _ret = file.read_to_string(&mut data);
    println!("{}", data);
    Ok(())
}
