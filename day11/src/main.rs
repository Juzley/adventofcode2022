use num::integer::lcm;
use std::collections::VecDeque;

#[derive(Clone, Debug)]
struct Monkey {
    items: VecDeque<u64>,
    operation: fn(u64) -> u64,
    mod_value: u64,
    on_true: usize,
    on_false: usize,
}

fn monkey_business(monkeys: &mut Vec<Monkey>, rounds: usize, worry_reduction: u64) -> u64 {
    // In order to keep the numbers in range for part 2, we can take the modulo of each item
    // with the LCM of all the monkey's check modulo values - this doesn't affect the result
    // of each move.
    let mod_multiple: u64 = monkeys
        .iter()
        .fold(1, |acc, monkey| lcm(acc, monkey.mod_value));

    let mut counts = vec![0; monkeys.len()];
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            loop {
                if let Some(mut item) = monkeys[i].items.pop_front() {
                    counts[i] += 1;

                    item = (monkeys[i].operation)(item) / worry_reduction;
                    item %= mod_multiple;

                    let throw_at;
                    if item % monkeys[i].mod_value == 0 {
                        throw_at = monkeys[i].on_true;
                    } else {
                        throw_at = monkeys[i].on_false;
                    }

                    monkeys[throw_at].items.push_back(item);
                } else {
                    break;
                }
            }
        }
    }

    counts.sort_by(|a, b| b.cmp(a));
    return counts.iter().take(2).product();
}

fn part1(mut monkeys: Vec<Monkey>) -> u64 {
    return monkey_business(&mut monkeys, 20, 3);
}

fn part2(mut monkeys: Vec<Monkey>) -> u64 {
    return monkey_business(&mut monkeys, 10000, 1);
}

fn main() {
    let monkeys = vec![
        Monkey {
            items: VecDeque::from([78, 53, 89, 51, 52, 59, 58, 85]),
            operation: |i| i * 3,
            mod_value: 5,
            on_true: 2,
            on_false: 7,
        },
        Monkey {
            items: VecDeque::from([64]),
            operation: |i| i + 7,
            mod_value: 2,
            on_true: 3,
            on_false: 6,
        },
        Monkey {
            items: VecDeque::from([71, 93, 65, 82]),
            operation: |i| i + 5,
            mod_value: 13,
            on_true: 5,
            on_false: 4,
        },
        Monkey {
            items: VecDeque::from([67, 73, 95, 75, 56, 74]),
            operation: |i| i + 8,
            mod_value: 19,
            on_true: 6,
            on_false: 0,
        },
        Monkey {
            items: VecDeque::from([85, 91, 90]),
            operation: |i| i + 4,
            mod_value: 11,
            on_true: 3,
            on_false: 1,
        },
        Monkey {
            items: VecDeque::from([67, 96, 69, 55, 70, 83, 62]),
            operation: |i| i * 2,
            mod_value: 3,
            on_true: 4,
            on_false: 1,
        },
        Monkey {
            items: VecDeque::from([53, 86, 98, 70, 64]),
            operation: |i| i + 6,
            mod_value: 7,
            on_true: 7,
            on_false: 0,
        },
        Monkey {
            items: VecDeque::from([88, 64]),
            operation: |i| i * i,
            mod_value: 17,
            on_true: 2,
            on_false: 5,
        },
    ];

    let pt1_result = part1(monkeys.clone());
    let pt2_result = part2(monkeys);

    println!("Part 1: {}: Part 2: {}", pt1_result, pt2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let monkeys = vec![
            Monkey {
                items: VecDeque::from([79, 98]),
                operation: |i| i * 19,
                mod_value: 23,
                on_true: 2,
                on_false: 3,
            },
            Monkey {
                items: VecDeque::from([54, 65, 75, 74]),
                operation: |i| i + 6,
                mod_value: 19,
                on_true: 2,
                on_false: 0,
            },
            Monkey {
                items: VecDeque::from([79, 60, 97]),
                operation: |i| i * i,
                mod_value: 13,
                on_true: 1,
                on_false: 3,
            },
            Monkey {
                items: VecDeque::from([74]),
                operation: |i| i + 3,
                mod_value: 17,
                on_true: 0,
                on_false: 1,
            },
        ];

        let result = part1(monkeys);
        assert_eq!(result, 10605)
    }

    #[test]
    fn pt2_test() {
        let monkeys = vec![
            Monkey {
                items: VecDeque::from([79, 98]),
                operation: |i| i * 19,
                mod_value: 23,
                on_true: 2,
                on_false: 3,
            },
            Monkey {
                items: VecDeque::from([54, 65, 75, 74]),
                operation: |i| i + 6,
                mod_value: 19,
                on_true: 2,
                on_false: 0,
            },
            Monkey {
                items: VecDeque::from([79, 60, 97]),
                operation: |i| i * i,
                mod_value: 13,
                on_true: 1,
                on_false: 3,
            },
            Monkey {
                items: VecDeque::from([74]),
                operation: |i| i + 3,
                mod_value: 17,
                on_true: 0,
                on_false: 1,
            },
        ];

        let result = part2(monkeys);
        assert_eq!(result, 2713310158);
    }
}
