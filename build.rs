use std::{
    fs::{self, OpenOptions},
    io::Write,
};

const UI_FILE: &'static str = "ui/app.slint";

fn main() {
    // Replace ui contents.
    let contents = fs::read_to_string(UI_FILE).unwrap();
    let new_contents = contents.replace("{{version}}", env!("CARGO_PKG_VERSION"));

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(UI_FILE)
        .unwrap();

    file.write(new_contents.as_bytes()).unwrap();

    // Build ui files.
    slint_build::compile(UI_FILE).unwrap();
}
