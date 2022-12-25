use crate::config::Config;

use super::container;

pub fn run_all_jobs(config: &Config) {
    let container_process = container::start_container("nginx", "nginx-ci");
    if let Err(err) = container_process {}
}
