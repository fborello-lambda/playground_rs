pub trait Algo<I, O> {
    fn run_algo(&mut self, input: I) -> O;
    fn implementation(&mut self) -> O;
}

pub mod example_algo;
pub mod rpn_interpreter;
