use std::collections::{HashMap, LinkedList};
use std::str::FromStr;
use placeholder::render; // todo: replace placeholder with std::fmt::format
use crate::xyz_file::XyzFile;
use super::xyz_file;

type SpaceCoordinate = (f64, f64, f64);
type Atom = (char, SpaceCoordinate);

#[derive(Clone)]
pub struct Molecule {
    pub atoms: LinkedList<Atom>,
}

impl Molecule {
    pub fn new() -> Molecule {
        Molecule{atoms: LinkedList::new()}
    }

    pub fn shift(molecule: &Molecule, distance: (f64, f64, f64)) -> Molecule {
        let mut atoms: LinkedList<Atom> = LinkedList::new();
        for atom in molecule.atoms.clone() {
            let shifted_atom: Atom = (atom.0, (atom.1.0 + distance.0, atom.1.1 + distance.1, atom.1.2 + distance.2));
            atoms.push_back(shifted_atom);
        }
        Molecule{atoms}
    }
}

impl From<&str> for Molecule {
    fn from(input: &str) -> Self {
        let mut molecule: Molecule = Molecule::new();
        let lines: Vec<&str> = input.split("\n").collect();
        for line in lines {
            let mut values: Vec<&str> = line.split_whitespace().collect();
            if values.len() == 4 { // Symbol \t x_coord \t y_coord \t z_coord
                let z = values.pop().unwrap().parse().unwrap();
                let y = values.pop().unwrap().parse().unwrap();
                let x = values.pop().unwrap().parse().unwrap();
                
                let space_coord: SpaceCoordinate = (x, y, z);

                let atom: Atom = (values.pop().unwrap().parse().unwrap(), space_coord);
                molecule.atoms.push_back(atom);
            }
        }
        molecule
    }
}

impl ToString for Molecule {
    /// A tab seperated string representation of a molecule. One line for each atom.
    fn to_string(&self) -> String {
        let mut result = String::new();
        for atom in self.atoms.clone() {
            let molecule_template = String::from("{symbol_atom} {x} {y} {z} \n");
            let mut values: HashMap<String, String> = HashMap::new();
            values.insert(String::from("symbol_atom"), String::from(atom.0));
            values.insert(String::from("x"), atom.1.0.to_string());
            values.insert(String::from("y"), atom.1.1.to_string());
            values.insert(String::from("z"), atom.1.2.to_string());
            result.push_str(&render(&molecule_template, &values).unwrap());
        }
        return result;
    }
}

impl XyzFile for Molecule {
    /// Returns a String in the format of xyz files for one molecule
    fn to_xyz_string(&self) -> String {
        let mut result = String::new();
        result.push_str(&*self.atoms.len().to_string()); // number of atoms
        result.push_str("\n"); // new line
        result.push_str("\n"); // empty line
        result.push_str(&*self.to_string()); // tab seperated molecule string
        result
    }
}