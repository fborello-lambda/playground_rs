use crate::algos::Algo;

// Define a struct with the Input and Output structs
#[derive(Default, Debug)]
pub struct ExampleAlgo {
    input: Input,
    output: Output,
}

#[derive(Default, Debug)]
pub struct Input {
    array: Vec<i32>,
    pos: i32,
}

// Define the new() function;
impl Input {
    pub fn new() -> Self {
        Input {
            array: vec![1, 3, 5, 6],
            pos: 5,
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Output {
    solution: i32,
}

impl Algo<Input, Output> for ExampleAlgo {
    fn run_algo(&mut self, input: Input) -> Output {
        self.input = input;
        self.implementation();

        self.output
    }

    fn implementation(&mut self) -> Output {
        self.output.solution = self
            .input
            .array
            .binary_search(&self.input.pos)
            .unwrap_or_else(|x| x)
            .try_into()
            .unwrap();

        self.output
    }
}

#[test]
fn test() {
    let input = Input::new();
    let mut algo = ExampleAlgo::default();
    algo.run_algo(input);

    assert_eq!(algo.output.solution, 2);
}
