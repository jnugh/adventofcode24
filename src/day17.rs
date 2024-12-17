use core::panic;

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Operator {
    DivisionA,
    DivisionB,
    DivisionC,
    Xor,
    Mod8,
    JumpNonZero,
    RegisterXOR,
    Mod8Output,
}

#[derive(Debug, Clone)]
struct Instruction {
    operator: Operator,
    operand: u8,
}

#[derive(Debug, Clone)]
struct Computer {
    a: u64,
    b: u64,
    c: u64,

    ip: usize,

    instructions: Vec<Instruction>,
    code: Vec<u64>,
    output: Vec<u64>,
}

impl Computer {
    fn from_input(input: &str) -> Self {
        let lines: Vec<&str> = input
            .split("\n")
            .map(|l| l.split(":").nth(1))
            .filter(|val| val.is_some())
            .map(|val| val.unwrap().trim())
            .collect();

        let a = lines[0].parse::<u64>().unwrap();
        let b = lines[1].parse::<u64>().unwrap();
        let c = lines[2].parse::<u64>().unwrap();

        let instructions = lines[3]
            .split(",")
            .chunks(2)
            .into_iter()
            .map(|x| {
                let values: Vec<&str> = x.collect();
                (values[0], values[1])
            })
            .map(|(operator, operand)| Instruction {
                operator: operator.into(),
                operand: operand.parse().unwrap(),
            })
            .collect();

        Self {
            a,
            b,
            c,
            ip: 0,
            instructions,
            output: Vec::new(),
            code: lines[3].split(",").map(|s| s.parse().unwrap()).collect(),
        }
    }

    fn evaluate_next_output(&mut self) -> Option<u64> {
        let instructions = self.instructions.clone();
        while let Some(instruction) = instructions.get(self.ip / 2) {
            match (&instruction.operator, instruction.operand) {
                (Operator::DivisionA, operand) => self.div_a(operand),
                (Operator::DivisionB, operand) => self.div_b(operand),
                (Operator::DivisionC, operand) => self.div_c(operand),
                (Operator::Xor, operator) => self.b ^= operator as u64,
                (Operator::RegisterXOR, _) => self.b ^= self.c,
                (Operator::Mod8, operand) => self.b = self.parse_combo_operand(operand) % 8,
                (Operator::Mod8Output, operand) => {
                    let out = self.parse_combo_operand(operand) % 8;
                    self.output.push(out);
                    self.ip += 2;
                    return Some(out);
                }
                (Operator::JumpNonZero, _) => {}
            }

            if instruction.operator == Operator::JumpNonZero && self.a != 0 {
                self.ip = instruction.operand as usize;
            } else {
                self.ip += 2;
            }
        }
        None
    }

    fn evaluate(&mut self) {
        while self.evaluate_next_output().is_some() {}
    }

    /*
     * The adv instruction (opcode 0) performs division. The numerator is the value in the A register.
     * The denominator is found by raising 2 to the power of the instruction's combo operand.
     * (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.)
     * The result of the division operation is truncated to an integer and then written to the A register.
     */
    fn div_a(&mut self, operand: u8) {
        self.a /= self.save_pow(operand);
    }

    fn div_b(&mut self, operand: u8) {
        self.b = self.a / self.save_pow(operand);
    }

    fn div_c(&mut self, operand: u8) {
        self.c = self.a / self.save_pow(operand);
    }

    fn save_pow(&self, operand: u8) -> u64 {
        let operand = self.parse_combo_operand(operand);
        if operand > 32 {
            u64::MAX
        } else {
            2_u64.pow(operand as u32)
        }
    }

    fn parse_combo_operand(&self, operand: u8) -> u64 {
        match operand {
            0..=3 => operand as u64,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("Invalid combo operand {}", operand),
        }
    }
}

impl From<&str> for Operator {
    fn from(value: &str) -> Self {
        Operator::from(value.parse::<u8>().unwrap())
    }
}

impl From<u8> for Operator {
    fn from(value: u8) -> Self {
        match value {
            0 => Operator::DivisionA,
            1 => Operator::Xor,
            2 => Operator::Mod8,
            3 => Operator::JumpNonZero,
            4 => Operator::RegisterXOR,
            5 => Operator::Mod8Output,
            6 => Operator::DivisionB,
            7 => Operator::DivisionC,
            _ => panic!("Unknown instruction {}", value),
        }
    }
}

pub fn day17_part1(input: String) -> String {
    let mut computer = Computer::from_input(&input);

    computer.evaluate();

    computer.output.into_iter().join(",").to_string()
}

pub fn day17_part2(input: String) -> usize {
    let computer = Computer::from_input(&input);

    let target_len = computer.code.len();
    let mut a = 0;
    let mut max_match = 0;
    loop {
        let mut computer = computer.clone();
        computer.a = a;

        computer.evaluate();
        let matches = computer
            .output
            .iter()
            .rev()
            .zip(computer.code.iter().rev())
            .take_while(|(a, b)| **a == **b)
            .count();
        if matches == target_len {
            return a as usize;
        }
        if matches > max_match {
            a <<= 3 * (matches - max_match);
            max_match = matches;
        } else {
            a += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::day17::{day17_part1, day17_part2, Computer};

    #[test]
    fn test_part1() {
        assert_eq!(
            "4,6,3,5,6,3,5,2,1,0",
            &day17_part1(
                r#"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"#
                    .to_string()
            )
        );
    }

    #[test]
    fn test_example1() {
        let mut computer = Computer::from_input(
            r#"Register A: 0
Register B: 0
Register C: 9

Program: 2,6"#,
        );
        computer.evaluate();
        assert_eq!(1, computer.b);
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            "0,1,2",
            &day17_part1(
                r#"Register A: 10
Register B: 0
Register C: 0

Program: 5,0,5,1,5,4"#
                    .to_string()
            )
        );
    }

    #[test]
    fn test_example3() {
        let mut computer = Computer::from_input(
            r#"Register A: 2024
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"#,
        );
        computer.evaluate();
        assert_eq!(0, computer.a);
        assert_eq!(vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0], computer.output);
    }

    #[test]
    fn test_example4() {
        let mut computer = Computer::from_input(
            r#"Register A: 0
Register B: 29
Register C: 0

Program: 1,7"#,
        );
        computer.evaluate();
        assert_eq!(26, computer.b);
    }

    #[test]
    fn test_example5() {
        let mut computer = Computer::from_input(
            r#"Register A: 0
Register B: 2024
Register C: 43690

Program: 4,0"#,
        );
        computer.evaluate();
        assert_eq!(44354, computer.b);
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            117440,
            day17_part2(
                r#"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"#
                    .to_string()
            )
        );
    }
}
