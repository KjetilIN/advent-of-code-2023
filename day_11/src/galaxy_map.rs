struct GalaxyMap{
    map: Vec<char>,
    galaxy_indexes: Vec<u32>
}

trait GalaxyMethods {
    fn from_file(content: String) -> Result<Self, String> where Self: Sized;
    fn expand_map(&mut self);
    fn set_galaxy_indexes(&mut self);
}