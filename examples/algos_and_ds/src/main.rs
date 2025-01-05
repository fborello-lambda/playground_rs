pub mod algos;

use algos::Algo;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = algos::example_algo::Input::new();
    let mut algo = algos::example_algo::ExampleAlgo::default();
    algo.run_algo(input);

    Ok(())
}
