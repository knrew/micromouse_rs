pub fn process(program: &str, args: &Vec<&str>, dir: &str) -> Result<std::process::ExitStatus, std::io::Error> {
    match match std::process::Command::new(program)
        .args(args)
        .current_dir(dir)
        .stdout(std::process::Stdio::inherit())
        .spawn() {
        Ok(o) => o,
        Err(e) => return Err(e),
    }.wait() {
        Ok(o) => Ok(o),
        Err(e) => Err(e)
    }
}
