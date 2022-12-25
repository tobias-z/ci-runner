use runtime::job;

pub mod config;
pub mod constants;
pub mod runtime;
pub mod types;

// CI Runner is run in a directory
// 1. Check if we have a ci file
// 2. Load and validate the configuration
// 3. If a main container runtime is choosen we run it
// 4. Figure out which steps should be run.
// 5. Go through the steps.
//  Example step:
//      1. Run in either the main container or in a specific one choosen for this step.
//      2. Figure our which execution script should be used depending on the configuration.
//          Shell, custom logic somehow, other ci file, etc.

fn main() {
    let configuration = config::load_config();
    if let Err(err) = configuration {
        println!("There was an error in your ci configuration");
        panic!("{}", err);
    }
    let configuration = configuration.unwrap();
    if configuration.is_valid_config() {
        job::run_all_jobs(&configuration);
    }
}
