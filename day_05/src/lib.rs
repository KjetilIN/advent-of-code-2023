pub mod main;

#[cfg(test)]
pub mod tests {

    use std::process::exit;

    use crate::main::{self, Almanac, AlmanacRange};

    const MAP: &str = "seeds: 79 14 55 13
seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_constructor(){
        let content: String = MAP.to_string();

        let almanac: Result<Almanac, String> = Almanac::with_list(&content);

        match almanac {
            Ok(_) => {},
            Err(_) => {
                eprintln!("Contractor test failed");
                assert_eq!(true, false);
            },
        }
    }

    #[test]
    fn test_get_dest_from_src(){
        let al_range = AlmanacRange::new(50, 98, 2);

        assert_eq!(al_range.get_dest_from_source(70), 70);
        assert_eq!(al_range.get_dest_from_source(98), 50);
        assert_eq!(al_range.get_dest_from_source(99), 51);
        assert_eq!(al_range.get_dest_from_source(100), 100);

    }

    #[test]
    fn test_get_smallest_location(){
        let content: String = MAP.to_string();

        let almanac = match Almanac::with_list(&content){
            Ok(a) => a,
            Err(_) => {
                eprintln!("Contractor test failed");
                exit(1)
            },
        };

        // The smallest for the given example should be 35
        let smallest = almanac.find_smallest_destination_number();
        assert_eq!(smallest, 35)
    }

}