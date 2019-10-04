use micromouse_rs::*;

mod parameters;
use parameters::*;

fn main() {
//    let params = parameters::Parameters::new();
    bash_process::process("mkdir", &["-p", &build_dir() as &str].to_vec(), "./").expect("failed to mkdir build");
    bash_process::process("cmake", &[&solver_dir() as &str].to_vec(), &build_dir()).expect("failed to cmake");
    bash_process::process("make", &[].to_vec(), &build_dir()).expect("failed to make");
}
