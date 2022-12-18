use std::collections::HashSet;

type Worry = u64;

#[derive(Debug)]
struct Throw {
    to: usize,
    what: usize,
}

#[derive(Debug)]
struct Monkey {
    items: HashSet<usize>, // indices into my inventory array
    oper: (String, String),
    test_factor: Worry,
    true_to: usize,  // index to monkey to throw to when condition true
    false_to: usize, // index to monkey to throw to when condition false
    business_conducted: u64,
}

impl Monkey {
    fn new(
        item_indices: Vec<usize>,
        operation: String,
        test_factor: Worry,
        true_to: usize,
        false_to: usize,
        monkey_factors: &mut Worry,
    ) -> Monkey {
        let operation = match operation.split_once("= old ").unwrap().1.split_once(' ') {
            Some(oper) => (oper.0.to_string(), oper.1.to_string()),
            _ => panic!("Invalid monkey business"),
        };

        *monkey_factors *= test_factor;

        Monkey {
            items: HashSet::from_iter(item_indices),
            test_factor,
            oper: operation,
            true_to,
            false_to,
            business_conducted: 0,
        }
    }

    fn business(&mut self, inventory: &mut Vec<Worry>, monkey_factors: &Worry) -> Vec<Throw> {
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

            inventory[item] %= monkey_factors;

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

pub fn execute() {
    let inputs = crate::start_day::setup("11");
    let mut inputs = inputs.iter();

    let mut inventory: Vec<Worry> = Vec::new();
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut monkey_factors: Worry = 1;

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
                inventory.push(
                    item.parse::<Worry>().expect("Invalid monkey data: item[n]"),
                );
                item_indices.push(inventory.len() - 1);
            }

            monkeys.push(Monkey::new(
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
                &mut monkey_factors,
            ));
        }
    }

    for _ in 0..10000 {
        for m in 0..monkeys.len() {
            for throw in monkeys[m].business(&mut inventory, &monkey_factors) {
                monkeys[throw.to].items.insert(throw.what);
            }
        }
    }

    monkeys.sort_by(|a, b| b.business_conducted.cmp(&a.business_conducted));

    println!(
        "Part 2: {}",
        monkeys[0].business_conducted * monkeys[1].business_conducted
    );
}
