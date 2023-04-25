use std::{env, fs};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use crate::molecule::Molecule;
use crate::pi_stack::{PiStack, PiStackOption};
use crate::xyz_file::XyzFile;

mod xyz_file;
mod molecule;
mod pi_stack;

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() >= 1 {
        for i in 1..args.len() {
            let data = fs::read_to_string(Path::new(&args[i])).unwrap();
            let molecule = Molecule::from(&*data);
            let stackoption = PiStackOption{conformation: 'E', multiply: 3, shift: (3.3, 3.3, 3.35)};
            let pistack = PiStack::from((&molecule, &stackoption));
            let xyz = pistack.to_xyz_string();
            File::create(stackoption.multiply.to_string()+"_" + &args[i]).unwrap().write_all(xyz.as_bytes()).unwrap();
        }


    }

    /*let data = fs::read_to_string(Path::new("naphthalene.xyz"));
    let data_string = data.unwrap();
    let molecule = Molecule::from(&*data_string);
    let stackoption = PiStackOption{conformation: 'E', multiply: 5, shift: (3.3, 3.3, 3.35)};
    let pistack = PiStack::from((&molecule, &stackoption));
    let xyz_string = pistack.to_xyz_string();

    let mut xyz_file = File::create("pi_stack.xyz")?;
    xyz_file.write_all(xyz_string.as_bytes())?;
    Ok(())*/
}
