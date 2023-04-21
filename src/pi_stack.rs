use std::collections::{HashMap, LinkedList};
use std::io::BufWriter;
use std::ops::Add;
use std::str::FromStr;
use placeholder::render;
use crate::xyz_file::XyzFile;
use crate::molecule::Molecule;

pub struct PiStackOption {
    pub conformation: char,
    pub multiply: usize,
    pub shift: (f64, f64, f64), // (x, y, z)
}

pub struct PiStack {
    pub molecules: LinkedList<Molecule>
}

impl PiStack {
    pub fn new() -> PiStack {
        PiStack{molecules: LinkedList::new()}
    }
}

impl XyzFile for PiStack {
    // this one solves to a dynamic-like xyz structure
    /*fn to_xyz_string(&self) -> String {
        let mut result = String::new();
        for molecule in self.molecules.clone() {
            result.push_str(&*molecule.to_xyz_string());
            //result.push_str("\n"); //molecule my molecule
        }
        result
    }*/
    fn to_xyz_string(&self) -> String {
        let mut result = String::new();
        let mut number_atoms: usize = 0;
        for molecule in self.molecules.clone() {
            number_atoms += molecule.atoms.len();
            result.push_str(&*molecule.to_string());
        }
        number_atoms.to_string() + "\n \n" + &*result.to_string()
    }
}

impl From<(& Molecule, &PiStackOption)> for PiStack  {
    fn from(input: (&Molecule, &PiStackOption)) -> PiStack {
        let mut result = PiStack::new();
        result.molecules.push_back(input.0.clone());

        if input.1.conformation == 'E' {
            for n in 1..input.1.multiply {
                let shift = (0., 0., n as f64*input.1.shift.2); //todo: shift also other coordinates
                result.molecules.push_back(Molecule::shift(input.0, shift));

                /*let mut shifted_molecule = Molecule::new();
                Molecule::shift(input.0, &mut shifted_molecule, (input.1.distance.0, 0., 0.));
                result.molecules.push_front(&shifted_molecule);*/
            }
        }
        result
    }
}