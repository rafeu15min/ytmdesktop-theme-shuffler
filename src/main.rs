use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
use std::{fs, io};
use rand::prelude::*;

fn main() -> std::io::Result<()>{
    let mut rng = rand::thread_rng();
    if !fs::exists("final-theme.css").is_err(){
        fs::remove_file("final-theme.css")?;
    }
    let mut entries = fs::read_dir("sorting")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;
    entries.shuffle(&mut rng);
    let mut target = File::create_new("final-theme.css")?;
    let mut source = File::open(entries.clone()[0].as_path())?;
    let mut content = String::new();
    source.read_to_string(&mut content)?;
    target.write_all(content.as_bytes())?;
    Ok(())
}
