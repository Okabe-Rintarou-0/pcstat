use std::{error::Error, process::{Command, Stdio}};

pub fn get_container_pid(id_or_name: &str) -> Result<usize, Box<dyn Error>> {
    let output = Command::new("docker")
        .args(["inspect", "-f", "{{.State.Pid}}", id_or_name])
        .stdout(Stdio::piped())
        .spawn()?;

    let output = output.wait_with_output()?;
    let output = String::from_utf8(output.stdout)?;
    let pid = output.trim().parse()?;
    Ok(pid)
}
