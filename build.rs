use std::{error::Error, fs};

pub const BG_CHAR: &str = "\u{00FF}";
pub const FG_CHAR: &str = "\u{00FE}";

const BG_TMPL_CHAR: char = '-';
const FG_TMPL_CHAR: char = '*';

fn main() -> Result<(), Box<dyn Error>> {
    for file in fs::read_dir("assets")? {
        let file = file?;
        if file.file_type()?.is_file() {
            let data = fs::read_to_string(file.path())?;
            let mut path = file.path();
            let stem = path
                .file_stem()
                .ok_or("cannot get file stem somehow")?
                .to_str()
                .ok_or("cannot convert to str")?
                .to_string();
            path.pop();

            let bin = path.join("bin");

            if !bin.exists() {
                fs::create_dir(bin)?;
            }

            fs::write(
                path.join("bin").join(format!("{}.bin", stem)),
                data.replace(BG_TMPL_CHAR, BG_CHAR)
                    .replace(FG_TMPL_CHAR, FG_CHAR),
            )?;
        }
    }

    Ok(())
}
