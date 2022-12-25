use std::{io, process};

pub fn start_container(
    image_name: &str,
    container_name: &str,
) -> Result<process::Output, io::Error> {
    process::Command::new("docker")
        .arg("run")
        .arg("--name")
        .arg(container_name)
        .arg("-d")
        // Provide the docker socket so that docker may be installed on whatever image is used.
        // This will allow the user to actually use docker correctly in the container if it is
        // installed
        .arg("-v")
        .arg("/var/run/docker.sock:/var/run/docker.sock")
        // Provide all the ci-runner data such as scripts and the workspace currently being
        // executed in
        .arg("-v")
        .arg("/ci-runner:/ci-runner")
        .arg(image_name)
        .output()
}

pub fn stop_container(container_name: &str) -> Result<process::Child, io::Error> {
    process::Command::new("docker")
        .arg("stop")
        .arg(container_name)
        .spawn()
}

pub fn exec_in_container(container_name: &str, cmd: &str) {}
