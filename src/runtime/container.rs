use std::{io, process};

pub fn start_container(
    image_name: &str,
    container_name: &str,
) -> Result<process::Child, io::Error> {
    process::Command::new("docker")
        .arg("run")
        .arg("--name")
        .arg(container_name)
        .arg("-d")
        .arg("-v")
        // Provide the docker socket so that docker may be installed on whatever image is used.
        // This will allow the user to actually use docker correctly in the container if it is
        // installed
        .arg("/var/run/docker.sock:/var/run/docker.sock")
        .arg("-v")
        .arg("/ci-runner:/ci-runner")
        .arg(image_name)
        .spawn()
}

pub fn stop_container(container_name: &str) -> Result<process::Child, io::Error> {
    process::Command::new("docker")
        .arg("stop")
        .arg(container_name)
        .spawn()
}

pub fn exec_in_container(container_name: &str, cmd: &str) {}
