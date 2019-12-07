struct Intcode {
    memory: Vec<i32>,
    pc: usize,
}

impl Intcode {
    pub fn new(input: &[i32]) -> Intcode {
        Intcode {
            memory: input.to_vec(),
            pc: 0,
        }
    }

    pub fn run(&mut self) -> i32 {
        while self.next() {}
        self.memory[0]
    }

    pub fn run_with(&mut self, noun: i32, verb: i32) -> i32 {
        self.memory[1] = noun;
        self.memory[2] = verb;
        self.run()
    }

    fn next(&mut self) -> bool {
        let opcode = self.memory[self.pc];
        match opcode {
            1 => {
                let pos1 = self.memory[self.pc + 1] as usize;
                let pos2 = self.memory[self.pc + 2] as usize;
                let pos_output = self.memory[self.pc + 3] as usize;
                self.memory[pos_output] = self.memory[pos1] + self.memory[pos2];
            }
            2 => {
                let pos1 = self.memory[self.pc + 1] as usize;
                let pos2 = self.memory[self.pc + 2] as usize;
                let pos_output = self.memory[self.pc + 3] as usize;
                self.memory[pos_output] = self.memory[pos1] * self.memory[pos2];
            }
            99 => return false,
            _ => unreachable!(),
        }
        self.pc += 4;
        true
    }
}

/// Parses each value to be an i32
#[aoc_generator(day2)]
fn generator_input(input: &str) -> Vec<i32> {
    input
        .split(',')
        .map(|a| a.parse::<i32>().unwrap())
        .collect()
}

#[aoc(day2, part1)]
fn solve_part1(input: &[i32]) -> i32 {
    let mut intcode = Intcode::new(input);
    intcode.run_with(12, 2)
}

#[aoc(day2, part2)]
fn solve_part2(input: &[i32]) -> i32 {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut intcode = Intcode::new(input);
            if intcode.run_with(noun, verb) == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples_part1() {
        let sample1 = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let sample1_res = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];
        let mut sample1_prog = Intcode::new(sample1.as_ref());
        assert_eq!(sample1_prog.run(), 3500);
        assert_eq!(sample1_prog.memory, sample1_res);

        let sample2 = vec![1, 0, 0, 0, 99];
        let sample2_res = vec![2, 0, 0, 0, 99];
        let mut sample2_prog = Intcode::new(sample2.as_ref());
        assert_eq!(sample2_prog.run(), 2);
        assert_eq!(sample2_prog.memory, sample2_res);

        let sample3 = vec![2, 3, 0, 3, 99];
        let sample3_res = vec![2, 3, 0, 6, 99];
        let mut sample3_prog = Intcode::new(sample3.as_ref());
        assert_eq!(sample3_prog.run(), 2);
        assert_eq!(sample3_prog.memory, sample3_res);

        let sample4 = vec![2, 4, 4, 5, 99, 0];
        let sample4_res = vec![2, 4, 4, 5, 99, 9801];
        let mut sample4_prog = Intcode::new(sample4.as_ref());
        assert_eq!(sample4_prog.run(), 2);
        assert_eq!(sample4_prog.memory, sample4_res);

        let sample5 = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let sample5_res = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];
        let mut sample5_prog = Intcode::new(sample5.as_ref());
        assert_eq!(sample5_prog.run(), 30);
        assert_eq!(sample5_prog.memory, sample5_res);
    }
}
