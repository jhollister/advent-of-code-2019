use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn get_fuel_by_mass(mass: i32) -> i32 {
    (mass / 3) - 2
}

fn get_total_fuel_by_mass(mass: i32) -> i32 {
    let fuel = get_fuel_by_mass(mass);
    if fuel <= 0 {
        return 0;
    }
    fuel + get_total_fuel_by_mass(fuel)
}

fn main() -> std::io::Result<()> {
    let file = File::open("input/day01.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let sum: i32 = contents
        .split_whitespace()
        .map(|x| get_total_fuel_by_mass(x.parse::<i32>().unwrap()))
        .sum();
    println!("Total fuel: {}", sum);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples_part1() {
        assert_eq!(get_fuel_by_mass(12), 2);
        assert_eq!(get_fuel_by_mass(14), 2);
        assert_eq!(get_fuel_by_mass(1969), 654);
        assert_eq!(get_fuel_by_mass(100756), 33583);
    }
    #[test]
    fn test_examples_part2() {
        assert_eq!(get_fuel_by_mass(12), 2);
        assert_eq!(get_fuel_by_mass(14), 2);
        assert_eq!(get_total_fuel_by_mass(100756), 50346);
        assert_eq!(get_total_fuel_by_mass(1969), 966);
    }
}
