use std::collections::HashSet;
use super::monkey::{Throw, Monkey};

type Worry = u64;

impl Monkey {
    fn new_a(
        item_indices: Vec<usize>,
        operation: String,
        test_factor: Worry,
        true_to: usize,
        false_to: usize,
    ) -> Monkey {
        Monkey {
            items: HashSet::from_iter(item_indices),
            oper: match operation.split_once("= old ").unwrap().1.split_once(' ') {
                Some(oper) => (oper.0.to_string(), oper.1.to_string()),
                _ => panic!("Invalid monkey business"),
            },
            test_factor,
            true_to,
            false_to,
            business_conducted: 0,
        }
    }

    fn business_a(&mut self, inventory: &mut Vec<Worry>) -> Vec<Throw> {
        let mut throws: Vec<Throw> = Vec::new();

        for item in self.items.drain() {
            self.business_conducted += 1;

            let n = match self.oper.1.parse::<Worry>() {
                Ok(n) => n,
                _ => inventory[item], // old
            };
            inventory[item] = match self.oper.0.as_str() {
                "+" => inventory[item] + n,
                "*" => inventory[item] * n,
                _ => panic!("Invalid monkey business attempted"),
            };

            inventory[item] /= 3;

            let target = if inventory[item] % self.test_factor == 0 {
                self.true_to
            } else {
                self.false_to
            };

            throws.push(Throw {
                to: target,
                what: item,
            });
        }

        throws
    }
}

pub fn execute(inputs: &Vec<String>) {
    let mut inputs = inputs.iter();

    let mut inventory: Vec<Worry> = Vec::new();
    let mut monkeys: Vec<Monkey> = Vec::new();

    while let Some(input) = inputs.next() {
        if input.starts_with("Monkey") {
            let (items, operation, test, true_to, false_to) = (
                inputs.next().expect("Monkey property undefined: items"),
                inputs.next().expect("Monkey property undefined: operation"),
                inputs.next().expect("Monkey property undefined: test"),
                inputs.next().expect("Monkey property undefined: if_true"),
                inputs.next().expect("Monkey property undefined: if_false"),
            );

            let mut starting_items = items.split(", ");
            inventory.push(
                starting_items
                    .next()
                    .unwrap()
                    .split_ascii_whitespace()
                    .last()
                    .unwrap()
                    .parse::<Worry>()
                    .expect("Invalid monkey data: items[0]"),
            );

            let mut item_indices = vec![inventory.len() - 1];
            while let Some(item) = starting_items.next() {
                inventory.push(item.parse::<Worry>().expect("Invalid monkey data: item[n]"));
                item_indices.push(inventory.len() - 1);
            }

            monkeys.push(Monkey::new_a(
                item_indices,
                operation.clone(),
                test.split_ascii_whitespace()
                    .last()
                    .unwrap()
                    .parse::<Worry>()
                    .unwrap(),
                true_to
                    .split_ascii_whitespace()
                    .last()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap(),
                false_to
                    .split_ascii_whitespace()
                    .last()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap(),
            ));
        }
    }

    for _ in 0..20 {
        for m in 0..monkeys.len() {
            for throw in monkeys[m].business_a(&mut inventory) {
                monkeys[throw.to].items.insert(throw.what);
            }
        }
    }

    monkeys.sort_by(|a, b| b.business_conducted.cmp(&a.business_conducted));

    println!(
        "Part 1: {}",
        monkeys[0].business_conducted * monkeys[1].business_conducted
    );
}
