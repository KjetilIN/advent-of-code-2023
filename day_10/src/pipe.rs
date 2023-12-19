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
        '-' => Pipe::Vertical,
        '|' => Pipe::Horizontal,
        'L' => Pipe::NorthWest,
        'J' => Pipe::NorthEast,
        '7' => Pipe::SouthWest,
        'F' => Pipe::SouthEast,
        '.' => Pipe::Ground,
        'S' => Pipe::Start,
        _ => Pipe::Undefined,
    }
}