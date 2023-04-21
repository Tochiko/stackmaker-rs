use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use crate::molecule::Molecule;
use crate::pi_stack::{PiStack, PiStackOption};
use crate::xyz_file::XyzFile;

mod xyz_file;
mod molecule;
mod pi_stack;

fn main() -> std::io::Result<()>{

    let data = fs::read_to_string(Path::new("test.xyz"));
    let data_string = data.unwrap();
    let molecule = Molecule::from(&*data_string);
    let stackoption = PiStackOption{conformation: 'E', multiply: 5, shift: (3.3, 3.3, 3.35)};
    let pistack = PiStack::from((&molecule, &stackoption));
    let xyz_string = pistack.to_xyz_string();

    let mut xyz_file = File::create("pi_stack.xyz")?;
    xyz_file.write_all(xyz_string.as_bytes())?;
    Ok(())
}
