use crate::algos::Algo;

// Define a struct with the Input and Output structs
#[derive(Default, Debug)]
pub struct RpnInterpreter<'a> {
    input: Input<'a>,
    output: Output,
}

#[derive(Debug, Clone, Copy)]
pub struct Input<'a> {
    expression: &'a str,
}

impl<'a> Default for Input<'a> {
    fn default() -> Self {
        Input {
            expression: "1+2+3+4+5",
        }
    }
}

// Define the new() function;
impl<'a> Input<'a> {
    pub fn new(expression: &'a str) -> Self {
        Input { expression }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Output {
    _solution: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}
impl<'a> Algo<Input<'a>, Output> for RpnInterpreter<'a> {
    fn run_algo(&mut self, input: Input<'a>) -> Output {
        self.input = input;
        self.implementation()
    }

    fn implementation(&mut self) -> Output {
        let (_solution, _) = calculate(self.input.expression);
        self.output = Output { _solution };
        self.output
    }
}

fn calculate(s: &str) -> (i32, usize) {
    let mut cur = 0_i32;
    let mut operation = Operation::Add;
    let mut stack = vec![];
    let mut i = 0;

    while i < s.len() {
        let c = s.chars().nth(i).unwrap();
        match c {
            '(' => {
                // The +1 is to avoid having the starting '('
                let (val, idx) = calculate(&s[i + 1..]);
                cur = val;
                // The +1 is to avoid having the ending ')'
                i += idx + 1;
            }
            ')' => {
                push_to_stack(operation, cur, &mut stack);
                return (stack.iter().sum(), i);
            }
            '0'..='9' => {
                cur = cur * 10 + (c.to_digit(10).unwrap()) as i32;
            }
            '+' | '-' | '*' | '/' => {
                push_to_stack(operation, cur, &mut stack);
                //println!("cur: {cur} || operation: {operation:?} || inner_stack: {stack:?}");
                match c {
                    '+' => operation = Operation::Add,
                    '-' => operation = Operation::Sub,
                    '*' => operation = Operation::Mul,
                    '/' => operation = Operation::Div,
                    _ => unreachable!(),
                }
                cur = 0;
            }
            _ => {
                //println!("Skipping unknown character {c}")
            }
        }
        i += 1;
    }
    push_to_stack(operation, cur, &mut stack);
    return (stack.iter().sum(), s.len() - 1);
}

fn push_to_stack(op: Operation, n: i32, stack: &mut Vec<i32>) {
    match op {
        Operation::Add => stack.push(n),
        Operation::Sub => stack.push(-n),
        Operation::Mul => {
            let last = stack.pop().unwrap();
            stack.push(last * n)
        }
        Operation::Div => {
            let last = stack.pop().unwrap();
            stack.push(last / n)
        }
    }
}

#[test]
pub fn test_rpn_interpreter() {
    let test_cases = [
        "1+2+33- 54",
        "(4+2*5+34/2-56)",
        "((1+5)-(6+2))",
        "(1-(4*2+5)-3)",
        "(5+5)/2",
    ];

    let results = [-18, -25, -2, -15, 5];

    for (case, result) in test_cases.iter().zip(results) {
        let mut algo = RpnInterpreter::default();
        let input = Input::new(case);
        let output = algo.run_algo(input);

        assert_eq!(output._solution, result)
    }
}
