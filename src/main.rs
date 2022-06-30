use anyhow::Result;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};

const SUDO_PATH: &str = "/etc/pam.d/sudo";
const TEMP_PATH: &str = "/tmp/enable-touch-id-sudo";

fn needs_configuration() -> Result<bool> {
    let file = File::open(SUDO_PATH)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if line?.contains("pam_tid.so") {
            return Ok(false);
        }
    }

    Ok(true)
}

fn create_file() -> Result<()> {
    let new_line = "auth       sufficient     pam_tid.so";

    let file = File::open(SUDO_PATH)?;
    let reader = BufReader::new(file);

    let mut temp_file = File::create(TEMP_PATH)?;

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        if i == 0 {
            if line.starts_with('#') {
                writeln!(temp_file, "{line}")?;
                writeln!(temp_file, "{new_line}")?;
            } else {
                writeln!(temp_file, "{new_line}")?;
                writeln!(temp_file, "{line}")?;
            }
        } else {
            writeln!(temp_file, "{line}")?;
        }
    }

    Ok(())
}

fn add_configuration() -> Result<()> {
    create_file()?;
    fs::copy(TEMP_PATH, SUDO_PATH)?;
    fs::remove_file(TEMP_PATH)?;
    Ok(())
}

fn main() -> Result<()> {
    if needs_configuration()? {
        add_configuration()?;
    }

    Ok(())
}
