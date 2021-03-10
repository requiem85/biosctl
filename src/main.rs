use anyhow::*;
use std::path::Path;

fn main() -> Result<()> {
    let path = std::env::args().nth(1).unwrap();
    let path = Path::new(&path);

    let attributes = firmconfig::list_attributes(&path)?;

    for a in attributes {
        println!("{:?}", a);
    }

    Ok(())
}
