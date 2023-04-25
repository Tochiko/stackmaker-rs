use std::{env, fs};
use std::fmt::format;
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
            let data = fs::read_to_string(Path::new(&format!("{}{}", "./", args[i]))).unwrap();
            let molecule = Molecule::from(&*data);
            let stackoption = PiStackOption{conformation: 'E', multiply: 3, shift: (3.3, 3.3, 3.35)};
            let pistack = PiStack::from((&molecule, &stackoption));
            let xyz = pistack.to_xyz_string();

            let mut output_filename = args[i].clone();
            output_filename.insert_str(output_filename.find(".").unwrap(),
                                       &*format!("{}{}", "_", stackoption.multiply.to_string()+ &*stackoption.conformation.to_string()));
            File::create(Path::new(&format!("{}{}", "./", output_filename)))
                .unwrap().write_all(xyz.as_bytes()).unwrap();
        }


    }
}
