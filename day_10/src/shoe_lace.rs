use crate::point::Point;

/// Calculates the area of a given polygon given a vector of points
/// Uses the shoe lace algorithm. Uses the Trapezoid formula
/// Source: https://en.wikipedia.org/wiki/Shoelace_formula  
pub fn find_area_from_points(points: &Vec<Point>) -> u32{
    let mut sum: i32 = 0;
    let n = points.len();
    for i in 0..n {
        let next_index = (i + 1) % n;
        sum += (points[i].x * points[next_index].y) - (points[next_index].x * points[i].y);
    }
    (sum/2).abs() as u32
}