use micromouse_rs::*;

mod parameters;

fn main() {
    let params = parameters::Parameters::new();
    bash_process::process("mkdir", &["-p", &params.build_dir].to_vec(), "./").expect("failed to mkdir build");
    bash_process::process("cmake", &[&params.solver_dir as &str].to_vec(), &params.build_dir).expect("failed to cmake");
    bash_process::process("make", &[].to_vec(), &params.build_dir).expect("failed to make");
}
