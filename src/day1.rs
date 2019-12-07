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

/// Parses each line to be an i32
#[aoc_generator(day1)]
fn generator_input(input: &str) -> Vec<i32> {
    input.lines().map(|a| a.parse::<i32>().unwrap()).collect()
}

#[aoc(day1, part1)]
fn solve_part1(input: &[i32]) -> i32 {
    input.iter().map(|x| get_total_fuel_by_mass(*x)).sum()
}

#[aoc(day1, part2)]
fn solve_part2(input: &[i32]) -> i32 {
    input.iter().map(|x| get_fuel_by_mass(*x)).sum()
}

#[cfg(test)]
mod tests {
    use super::{get_fuel_by_mass, get_total_fuel_by_mass};

    #[test]
    fn test_examples_part1() {
        assert_eq!(get_fuel_by_mass(12), 2);
        assert_eq!(get_fuel_by_mass(14), 2);
        assert_eq!(get_fuel_by_mass(1969), 654);
        assert_eq!(get_fuel_by_mass(100756), 33583);
    }
    #[test]
    fn test_examples_part2() {
        assert_eq!(get_total_fuel_by_mass(12), 2);
        assert_eq!(get_total_fuel_by_mass(14), 2);
        assert_eq!(get_total_fuel_by_mass(100756), 50346);
        assert_eq!(get_total_fuel_by_mass(1969), 966);
    }
}
