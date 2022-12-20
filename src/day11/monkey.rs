use std::collections::HashSet;

type Worry = u64;

#[derive(Debug)]
pub struct Throw {
    pub to: usize,
    pub what: usize,
}

#[derive(Debug)]
pub struct Monkey {
    pub items: HashSet<usize>, // indices into my inventory array
    pub oper: (String, String),
    pub test_factor: Worry,
    pub true_to: usize,  // index to monkey to throw to when condition true
    pub false_to: usize, // index to monkey to throw to when condition false
    pub business_conducted: u64,
}

