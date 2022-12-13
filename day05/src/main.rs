use regex::Regex;
use std::collections::BTreeMap;

#[derive(Clone)]
struct Stack(BTreeMap<u32, Vec<char>>);

#[derive(Copy, Clone)]
struct Instruction {
    times: u32,
    from: u32,
    to: u32,
}

impl Instruction {
    fn new(times: u32, from: u32, to: u32) -> Self {
        Self { times, from, to }
    }
}

impl Stack {
    fn calclulate_instruction_for_mover_9000(&mut self, instruction: Instruction) {
        for _ in 0..instruction.times {
            let x = self
                .0
                .get_mut(&instruction.from)
                .expect("we know the stacks")
                .pop()
                .expect("stack should not be empty");

            self.0
                .get_mut(&instruction.to)
                .expect("we know the stacks")
                .push(x);
        }
    }

    fn calclulate_instruction_for_mover_9001(&mut self, instruction: Instruction) {
        let old_stack = self
            .0
            .get_mut(&instruction.from)
            .expect("we know the stacks");
        let mut splitted = old_stack.split_off(old_stack.len() - instruction.times as usize);

        self.0
            .get_mut(&instruction.to)
            .expect("we know the stacks")
            .append(&mut splitted);
    }

    fn print(self) -> String {
        self.0.iter().map(|(_, v)| v.last().unwrap()).collect()
    }
}
fn main() {
    let mut stack = Stack(BTreeMap::from([
        (1, vec!['H', 'T', 'Z', 'D']),
        (2, vec!['Q', 'R', 'W', 'T', 'G', 'C', 'S']),
        (3, vec!['P', 'B', 'F', 'Q', 'N', 'R', 'C', 'H']),
        (4, vec!['L', 'C', 'N', 'F', 'H', 'Z']),
        (5, vec!['G', 'L', 'F', 'Q', 'S']),
        (6, vec!['V', 'P', 'W', 'Z', 'B', 'R', 'C', 'S']),
        (7, vec!['Z', 'F', 'J']),
        (8, vec!['D', 'L', 'V', 'Z', 'R', 'H', 'Q']),
        (9, vec!['B', 'H', 'G', 'N', 'F', 'Z', 'L', 'D']),
    ]));
    let mut stack2 = stack.clone();
    let input = include_str!("input.txt");
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let instructions = input
        .lines()
        .map(|line| {
            let captures = re.captures(line).expect("wrong line format");
            Instruction::new(
                captures[1].parse().expect("parseable"),
                captures[2].parse().expect("parseable"),
                captures[3].parse().expect("parseable"),
            )
        })
        .collect::<Vec<_>>();
    instructions
        .clone()
        .into_iter()
        .for_each(|instruction| stack.calclulate_instruction_for_mover_9000(instruction));

    println!(
        "The stack has the code '{}' when calculated for the mover 9000",
        stack.print()
    );

    instructions
        .clone()
        .into_iter()
        .for_each(|instruction| stack2.calclulate_instruction_for_mover_9001(instruction));

    println!(
        "The stack has the code '{}' when calculated for the mover 9001",
        stack2.print()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_9000() {
        let mut stack = Stack(BTreeMap::from([
            (1, vec!['Z', 'N']),
            (2, vec!['M', 'C', 'D']),
            (3, vec!['P']),
        ]));

        stack.calclulate_instruction_for_mover_9000(Instruction::new(1, 2, 1));
        stack.calclulate_instruction_for_mover_9000(Instruction::new(3, 1, 3));
        stack.calclulate_instruction_for_mover_9000(Instruction::new(2, 2, 1));
        stack.calclulate_instruction_for_mover_9000(Instruction::new(1, 1, 2));

        assert_eq!(stack.print(), "CMZ".to_string())
    }

    #[test]
    fn test_stack_9001() {
        let mut stack = Stack(BTreeMap::from([
            (1, vec!['Z', 'N']),
            (2, vec!['M', 'C', 'D']),
            (3, vec!['P']),
        ]));

        stack.calclulate_instruction_for_mover_9001(Instruction::new(1, 2, 1));
        stack.calclulate_instruction_for_mover_9001(Instruction::new(3, 1, 3));
        stack.calclulate_instruction_for_mover_9001(Instruction::new(2, 2, 1));
        stack.calclulate_instruction_for_mover_9001(Instruction::new(1, 1, 2));

        assert_eq!(stack.print(), "MCD".to_string())
    }
}
