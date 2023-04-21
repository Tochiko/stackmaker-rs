
pub trait XyzFile {
    /// Returns a String in the format of xyz files
    fn to_xyz_string(&self) -> String;
}