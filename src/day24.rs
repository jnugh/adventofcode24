use std::{
    cell::RefCell,
    cmp::Reverse,
    collections::{HashMap, HashSet},
};

use itertools::Itertools;
use log::warn;

#[derive(Clone, PartialEq, Eq, Debug)]
enum GateType {
    And,
    Or,
    Xor,
}

#[derive(Clone)]
struct Gate {
    gate_type: GateType,
    wire1: String,
    wire2: String,
}

impl Gate {
    fn parse(input: &str) -> Self {
        let parts = input
            .trim()
            .split(" ")
            .collect_tuple::<(_, _, _)>()
            .unwrap();

        match parts.1 {
            "AND" => Self {
                gate_type: GateType::And,
                wire1: parts.0.to_string(),
                wire2: parts.2.to_string(),
            },
            "OR" => Self {
                gate_type: GateType::Or,
                wire1: parts.0.to_string(),
                wire2: parts.2.to_string(),
            },
            "XOR" => Self {
                gate_type: GateType::Xor,
                wire1: parts.0.to_string(),
                wire2: parts.2.to_string(),
            },
            _ => panic!("Unknown gate type {}", parts.1),
        }
    }
}

#[derive(Clone)]
struct WireNetwork {
    wire_states: RefCell<HashMap<String, bool>>,
    gates: HashMap<String, Gate>,
    swaps: Vec<String>,
    swap_ideas: Vec<(String, String)>,
}

impl WireNetwork {
    fn from_input(input: &str) -> Self {
        let sections: Vec<_> = input.trim().split("\n\n").collect();
        let wires = sections[0]
            .lines()
            .flat_map(|line| line.split(": ").collect_tuple::<(_, _)>());
        let wire_states =
            HashMap::from_iter(wires.map(|(name, state)| (name.to_string(), state == "1")));

        let gates = sections[1]
            .lines()
            .flat_map(|line| line.split(" -> ").collect_tuple::<(_, _)>())
            .map(|(config, output_wire)| (output_wire.to_string(), Gate::parse(config)));
        let gates = HashMap::from_iter(gates);

        Self {
            wire_states: RefCell::new(wire_states),
            gates,
            swaps: Vec::new(),
            swap_ideas: Vec::new(),
        }
    }

    fn evaluate(&self, area: &str) -> usize {
        let mut result_bits: Vec<_> = self
            .gates
            .iter()
            .filter(|(gate, _)| gate.starts_with(area))
            .map(|(key, gate)| (key.clone(), self.evaluate_gate(gate)))
            .collect();

        result_bits.extend(
            self.wire_states
                .borrow()
                .iter()
                .filter(|(wire, _)| wire.starts_with(area))
                .map(|(wire, state)| (wire.clone(), *state)),
        );
        result_bits.sort_unstable_by_key(|(gate, _)| Reverse(gate.to_string()));

        let mut result = 0;
        for (_, bit) in result_bits {
            result <<= 1;
            if bit {
                result += 1;
            }
        }
        result
    }

    fn evaluate_gate(&self, gate: &Gate) -> bool {
        let w1 = self.evaluate_wire(&gate.wire1);
        let w2 = self.evaluate_wire(&gate.wire2);

        match gate.gate_type {
            GateType::And => w1 && w2,
            GateType::Or => w1 || w2,
            GateType::Xor => w1 ^ w2,
        }
    }

    fn evaluate_wire(&self, wire: &str) -> bool {
        if let Some(state) = self.wire_states.borrow().get(wire) {
            return *state;
        }

        let gate = self.gates.get(wire).unwrap();
        let result = self.evaluate_gate(gate);
        self.wire_states
            .borrow_mut()
            .insert(wire.to_string(), result);

        result
    }

    fn swap(&mut self, gate_a: &str, gate_b: &str) {
        warn!("Swapping {} AND {}", gate_a, gate_b);
        let content_a = self.gates.get(gate_a).unwrap().clone();
        self.gates
            .insert(gate_a.to_string(), self.gates.get(gate_b).unwrap().clone());
        self.gates.insert(gate_b.to_string(), content_a);
        self.swaps.push(gate_a.to_string());
        self.swaps.push(gate_b.to_string());
    }

    fn get_direct_overflow_gate(&mut self, n: usize) -> Option<String> {
        self.find_gate(
            GateType::And,
            &format!("x{:0>2}", n),
            &format!("y{:0>2}", n),
        )
    }

    fn get_direct_sum_gate(&mut self, n: usize) -> Option<String> {
        self.find_gate(
            GateType::Xor,
            &format!("x{:0>2}", n),
            &format!("y{:0>2}", n),
        )
    }

    fn get_carry_to_gate(&mut self, n: usize) -> Option<String> {
        if n == 0 {
            return None;
        }
        if n == 1 {
            return self.get_direct_overflow_gate(n - 1);
        }

        if let Some(direct_carry_gate) = self.get_direct_overflow_gate(n - 1) {
            if let Some(previous_carry) = self.get_overflow_from_carry_gate(n) {
                self.find_gate(GateType::Or, &direct_carry_gate, &previous_carry)
            } else {
                warn!("Missing previous carry gate for {}", n);
                None
            }
        } else {
            warn!("Missing direct carry gate for {}", n);
            None
        }
    }

    fn get_overflow_from_carry_gate(&mut self, n: usize) -> Option<String> {
        if n == 0 {
            return self.get_direct_overflow_gate(n);
        }
        if let Some(direct_dum_gate) = self.get_direct_sum_gate(n - 1) {
            if let Some(carry_to_gate) = self.get_carry_to_gate(n - 1) {
                self.find_gate(GateType::And, &direct_dum_gate, &carry_to_gate)
            } else {
                warn!("Missing direct carry gate for {}", n - 1);
                None
            }
        } else {
            warn!("Missing sum gate for {}", n);
            None
        }
    }

    fn get_sum_gate(&mut self, n: usize) -> Option<String> {
        if n == 0 {
            self.get_direct_sum_gate(n)
        } else if let Some(direct_sum) = self.get_direct_sum_gate(n) {
            if let Some(direct_carry) = self.get_carry_to_gate(n) {
                self.find_gate(GateType::Xor, &direct_sum, &direct_carry)
            } else {
                warn!("Missing direct_carry for {}", n - 1);
                None
            }
        } else {
            warn!("Missing direct sum for {}", n);
            None
        }
    }

    fn find_gate(&mut self, gate_type: GateType, w1: &str, w2: &str) -> Option<String> {
        if let Some(gate) = self
            .gates
            .iter()
            .find(|(_, gate)| {
                gate.gate_type == gate_type
                    && ((gate.wire1 == w1 && gate.wire2 == w2)
                        || (gate.wire1 == w2 && gate.wire2 == w1))
            })
            .map(|(name, _)| name.clone())
        {
            Some(gate)
        } else {
            warn!(
                "Could not find {:?} gate between {} and {}",
                gate_type, w1, w2
            );
            let relevant_gates = self.gates.iter().find(|(_, gate)| {
                gate.gate_type == gate_type
                    && (gate.wire1 == w1
                        || gate.wire2 == w1
                        || gate.wire1 == w2
                        || gate.wire2 == w2)
            });
            if let Some((_, relevant_gate)) = relevant_gates {
                if relevant_gate.wire1 == w1 {
                    self.swap_ideas
                        .push((w2.to_string(), relevant_gate.wire2.clone()));
                }
                if relevant_gate.wire2 == w1 {
                    self.swap_ideas
                        .push((w2.to_string(), relevant_gate.wire1.clone()));
                }
                if relevant_gate.wire1 == w2 {
                    self.swap_ideas
                        .push((w1.to_string(), relevant_gate.wire2.clone()));
                }
                if relevant_gate.wire2 == w2 {
                    self.swap_ideas
                        .push((w1.to_string(), relevant_gate.wire1.clone()));
                }
            }
            None
        }
    }
}

pub fn day24_part1(input: String) -> usize {
    let wire_network = WireNetwork::from_input(&input);

    wire_network.evaluate("z")
}

pub fn day24_part2(input: String) -> String {
    let mut wire_network = WireNetwork::from_input(&input);

    for i in 0..45 {
        if let Some(sum_gate) = wire_network.get_sum_gate(i) {
            if sum_gate != format!("z{:0>2}", i) {
                wire_network.swap(&sum_gate, &format!("z{:0>2}", i));
            }
        } else if let Some(mut result) =
            fix_gates_with_swaps(&mut wire_network, &mut HashSet::new())
        {
            result.sort();
            return result.join(",");
        }
    }

    "Failed to find solution :/".to_string()
}

fn fix_gates_with_swaps(
    wire_network: &mut WireNetwork,
    known_swaps: &mut HashSet<(String, String)>,
) -> Option<Vec<String>> {
    while let Some(swap_idea) = wire_network.swap_ideas.pop() {
        if known_swaps.contains(&swap_idea) {
            continue;
        }
        wire_network.swap(&swap_idea.0, &swap_idea.1);

        for i in 0..45 {
            if let Some(sum_gate) = wire_network.get_sum_gate(i) {
                if sum_gate != format!("z{:0>2}", i) {
                    wire_network.swap(&sum_gate, &format!("z{:0>2}", i));
                }
            } else {
                let mut new_wire_network = wire_network.clone();
                new_wire_network.swap_ideas.clear();
                fix_gates_with_swaps(&mut new_wire_network, known_swaps);

                return None;
            }
        }

        return Some(wire_network.swaps.clone());
    }

    None
}

#[cfg(test)]
mod test {
    use crate::day24::day24_part1;

    #[test]
    fn test_part1() {
        assert_eq!(
            2024,
            day24_part1(
                r#"x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj"#
                    .to_string()
            )
        )
    }
}
