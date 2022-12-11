use crate::puzzle::Puzzle;
use std::collections::HashMap;

pub struct DayEleven;

#[derive(Debug, Clone)]
enum OperationTarget {
    Number(usize),
    Old,
}

#[derive(Debug, Clone)]
enum Operation {
    Add(OperationTarget),
    Multiply(OperationTarget),
}

#[derive(Debug, Clone)]
struct TestCase {
    divisible_by: usize,
    true_target: usize,
    false_target: usize,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    test: TestCase,
}

fn parse_input(input: String) -> Vec<Monkey> {
    let mut monkeys = Vec::new();
    let monkey_data = input.split("\n\n").collect::<Vec<_>>();

    for data in monkey_data {
        let mut lines = data.lines().map(|x| x.trim());
        let _ = lines.next().unwrap();

        let starting_items = lines
            .next()
            .unwrap()
            .replace("Starting items: ", "")
            .split(", ")
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let operation = lines.next().unwrap().replace("Operation: new = old ", "");
        let operation = operation.split(' ').collect::<Vec<_>>();

        let operation_target = match operation[1] {
            "old" => OperationTarget::Old,
            _ => OperationTarget::Number(operation[1].parse::<usize>().unwrap()),
        };

        let operation = match operation[0] {
            "+" => Operation::Add(operation_target),
            "*" => Operation::Multiply(operation_target),
            _ => unreachable!(),
        };

        let test = lines
            .next()
            .unwrap()
            .replace("Test: divisible by ", "")
            .parse::<usize>()
            .unwrap();

        let test_if_true = lines
            .next()
            .unwrap()
            .replace("If true: throw to monkey ", "")
            .parse::<usize>()
            .unwrap();

        let test_if_false = lines
            .next()
            .unwrap()
            .replace("If false: throw to monkey ", "")
            .parse::<usize>()
            .unwrap();

        let monkey = Monkey {
            items: starting_items,
            operation,
            test: TestCase {
                divisible_by: test,
                true_target: test_if_true,
                false_target: test_if_false,
            },
        };

        monkeys.push(monkey);
    }

    monkeys
}

impl Puzzle for DayEleven {
    fn test(&self) -> (String, String) {
        ("10605".to_string(), "2713310158".to_string())
    }

    fn one(&self, input: String) -> String {
        let mut input = parse_input(input);
        let mut monkey_inspect_count = HashMap::new();

        for _ in 0..20 {
            for monkey_idx in 0..input.len() {
                let monkey = input[monkey_idx].clone();
                let mut inspect_count = *monkey_inspect_count.entry(monkey_idx).or_insert(0);

                for worry_level in monkey.items.iter() {
                    let mut worry_level = *worry_level;

                    match monkey.operation {
                        Operation::Add(OperationTarget::Number(num)) => worry_level += num,
                        Operation::Add(OperationTarget::Old) => worry_level += worry_level,
                        Operation::Multiply(OperationTarget::Number(num)) => worry_level *= num,
                        Operation::Multiply(OperationTarget::Old) => worry_level *= worry_level,
                    };

                    worry_level = ((worry_level as f64) / 3.).floor() as usize;

                    let target = if (worry_level % monkey.test.divisible_by) == 0 {
                        monkey.test.true_target
                    } else {
                        monkey.test.false_target
                    };

                    input[target].items.push(worry_level);
                    inspect_count += 1;
                }

                input[monkey_idx].items = Vec::new();
                monkey_inspect_count.insert(monkey_idx, inspect_count);
            }
        }

        let mut inspect_counts = monkey_inspect_count.values().collect::<Vec<_>>();
        inspect_counts.sort_by(|a, b| b.cmp(a));

        (inspect_counts[0] * inspect_counts[1]).to_string()
    }

    fn two(&self, input: String) -> String {
        let mut input = parse_input(input);
        let mut monkey_inspect_count = HashMap::new();

        let product: usize = input.iter().map(|x| x.test.divisible_by).product();

        for _ in 0..10000 {
            for monkey_idx in 0..input.len() {
                let monkey = input[monkey_idx].clone();
                let mut inspect_count: usize = *monkey_inspect_count.entry(monkey_idx).or_insert(0);

                for worry_level in monkey.items.iter() {
                    let mut worry_level = *worry_level;
                    match monkey.operation {
                        Operation::Add(OperationTarget::Number(num)) => worry_level += num,
                        Operation::Add(OperationTarget::Old) => worry_level += worry_level,
                        Operation::Multiply(OperationTarget::Number(num)) => worry_level *= num,
                        Operation::Multiply(OperationTarget::Old) => worry_level *= worry_level,
                    };

                    worry_level %= product;

                    let target = if (worry_level % monkey.test.divisible_by) == 0 {
                        monkey.test.true_target
                    } else {
                        monkey.test.false_target
                    };

                    input[target].items.push(worry_level);
                    inspect_count += 1;
                }

                input[monkey_idx].items = Vec::new();
                monkey_inspect_count.insert(monkey_idx, inspect_count);
            }
        }

        let mut inspect_counts = monkey_inspect_count.values().collect::<Vec<_>>();
        inspect_counts.sort_by(|a, b| b.cmp(a));

        (inspect_counts[0] * inspect_counts[1]).to_string()
    }
}
