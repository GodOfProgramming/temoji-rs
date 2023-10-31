use clap::Parser;
use rust_embed::RustEmbed;
use std::error::Error;

pub const BG_CHAR: &str = "\u{00FF}";
pub const FG_CHAR: &str = "\u{00FE}";

const HEIGHT: usize = 7;

#[derive(RustEmbed)]
#[folder = "assets/bin"]
struct Assets;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    text: String,
    #[arg(short, long)]
    foreground: String,
    #[arg(short, long)]
    background: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let text = args.text.to_lowercase();

    let mut letters = Vec::with_capacity(args.text.len());

    for c in text.chars() {
        let c_bytes = Assets::get(&format!("{}.bin", c))
            .or_else(|| Assets::get(&format!("0x{:x}.bin", c as u32)))
            .ok_or_else(|| format!("could not load file assets/{}.bin (0x{:x})", c, c as u32))?;

        let c_str = unsafe { String::from_utf8_unchecked(c_bytes.data.into()) };

        letters.push(
            c_str
                .lines()
                .map(|l| {
                    l.to_string()
                        .replace(BG_CHAR, &args.background)
                        .replace(FG_CHAR, &args.foreground)
                })
                .collect::<Vec<String>>(),
        );
    }

    for i in 0..HEIGHT {
        for letter in letters.iter() {
            print!("{}", letter[i]);
        }
        println!();
    }

    Ok(())
}
