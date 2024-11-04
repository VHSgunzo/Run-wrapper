use std::env;
use which::which;
use std::path::PathBuf;
use std::process::{exit, Command};
use std::os::unix::process::CommandExt;

static RED: &str = "\x1b[91m";
static RESETCOLOR: &str = "\x1b[0m";

fn dirname(path: &str) -> String {
    let mut pieces: Vec<&str> = path.split('/').collect();
    if pieces.len() == 1 || path.is_empty() {
        // return ".".to_string();
    } else if !path.starts_with('/') &&
        !path.starts_with('.') &&
        !path.starts_with('~') {
            pieces.insert(0, ".");
    } else if pieces.len() == 2 && path.starts_with('/') {
        pieces.insert(0, "");
    };
    pieces.pop();
    pieces.join(&'/'.to_string())
}

macro_rules! error_msg {
    ($msg:expr) => {{
        eprintln!("{}[ ERROR ]: {}{}", RED, $msg, RESETCOLOR);
    }};
}

fn main() {
    let self_exe = env::current_exe().unwrap();
    let self_exe_dir = self_exe.parent().unwrap();

    let mut exec_args: Vec<String> = env::args().collect();

    let arg0 = PathBuf::from(exec_args.remove(0));
    let arg0_name = arg0.file_name().unwrap();
    let arg0_str = arg0.to_str().unwrap();

    let arg0_dir = PathBuf::from(dirname(arg0_str)).canonicalize()
        .unwrap_or_else(|_|{
            if let Ok(which_arg0) = which(arg0_name) {
                which_arg0.parent().unwrap().to_path_buf()
            } else {
                error_msg!("Failed to find ARG0 dir!");
                exit(1)
            }
    });

    let static_bash = self_exe_dir.join("static").join("bash");
    if !static_bash.is_file() {
        error_msg!("Static bash not found!");
        exit(1)
    }
    exec_args.insert(0, format!("{}/Run.sh", self_exe_dir.display()));

    env::set_var("ARG0", arg0_str);
    env::set_var("RUNDIR", self_exe_dir);
    env::set_var("RUNSRC", arg0_dir.join(arg0_name));

    drop(arg0);
    drop(self_exe);
    drop(arg0_dir);

    let err = Command::new(static_bash).args(&exec_args).exec();
    error_msg!(err.to_string());
    exit(1)
}
