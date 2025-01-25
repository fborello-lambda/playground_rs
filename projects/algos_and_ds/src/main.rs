pub mod algos;

use algos::Algo;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = algos::example_algo::Input::new();
    let mut algo = algos::example_algo::ExampleAlgo::default();
    algo.run_algo(input);
    println!("{algo:#?}");

    let input = algos::rpn_interpreter::Input::default();
    let mut algo = algos::rpn_interpreter::RpnInterpreter::default();
    algo.run_algo(input);
    println!("{algo:#?}");

    Ok(())
}
