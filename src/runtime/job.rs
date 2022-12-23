use super::container;

pub fn run() {
    let container_process = container::start_container("nginx", "nginx-ci");
    if let Err(err) = container_process {}
}
