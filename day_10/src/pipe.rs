#[derive(Debug)]
pub enum Pipe{
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Ground,
    Start,
    Undefined
}


pub fn get_pipe(ch: char)-> Pipe{
    return match &ch {
        '-' => Pipe::Horizontal,
        '|' => Pipe::Vertical,
        'L' => Pipe::NorthEast,
        'J' => Pipe::NorthWest,
        '7' => Pipe::SouthWest,
        'F' => Pipe::SouthEast,
        '.' => Pipe::Ground,
        'S' => Pipe::Start,
        _ => Pipe::Undefined,
    }
}