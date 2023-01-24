#![allow(non_snake_case, dead_code)]
extern crate which;
extern crate chrono;
use std::env;
use which::which;
use chrono::Local;
use std::path::{Path, PathBuf};
use std::process::{exit, Command};
use std::os::unix::process::CommandExt;

static RED: &str = "\x1b[91m";
static BLUE: &str = "\x1b[94m";
static GREEN: &str = "\x1b[92m";
static YELLOW: &str = "\x1b[33m";
static RESETCOLOR: &str = "\x1b[0m";

pub fn basename(path: &str) -> String {
    let pieces: Vec<&str> = path.rsplit('/').collect();
    return pieces.get(0).unwrap().to_string();
}

pub fn dirname(path: &str) -> String {
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
    return pieces.join(&'/'.to_string());
}

pub fn get_env_var(env_var: &str) -> String {
    let mut ret = "".to_string();
    if let Ok(res) = env::var(env_var) { ret = res };
    return ret;
}

pub fn error_msg(msg: &str) {
    let date = Local::now().format("%Y.%m.%d %H:%M:%S");
    eprintln!("{}[ ERROR ][{}]: {}{}", RED, date, msg, RESETCOLOR);
}

pub fn info_msg(msg: &str) {
    if ! get_env_var("QUIET_MODE").eq("1") {
        let date = Local::now().format("%Y.%m.%d %H:%M:%S");
        println!("{}[ INFO ][{}]: {}{}", GREEN, date, msg, RESETCOLOR);
    };
}

pub fn warn_msg(msg: &str) {
    if ! get_env_var("QUIET_MODE").eq("1") {
        let date = Local::now().format("%Y.%m.%d %H:%M:%S");
        println!("{}[ WARNING ][{}]: {}{}", YELLOW, date, msg, RESETCOLOR);
    };
}

fn main() {
    let self_exe = env::current_exe().unwrap();
    let self_exe_dir = self_exe.parent().unwrap().to_str().unwrap();
    let mut exec_args: Vec<String> = env::args().collect();
    let argv0 = exec_args.remove(0);
    exec_args.insert(0, format!("{}{}Run.sh", self_exe_dir, '/'));
    let argv0_name = basename(&argv0);
    let mut which_argv0= PathBuf::new();
    let mut argv0_dir= PathBuf::new();
    if let Ok(res) = which(&argv0_name) { which_argv0 = res };
    if let Ok(res) = Path::new(&dirname(&argv0)).canonicalize() { argv0_dir = res }
    else if let Ok(res) = Path::new(&dirname(&which_argv0.as_os_str().to_str().unwrap()))
        .canonicalize() { argv0_dir = res };
    let argv0_path = format!("{}{}{}", argv0_dir.display(), '/', argv0_name);

    let static_bash = format!("{}{}static{}bash", self_exe_dir, '/', '/');
    let static_bash_path = Path::new(&static_bash);
    if ! static_bash_path.is_file() {
        error_msg(&format!("Static bash not found: '{}'", static_bash_path.display()));
        exit(1);
     };

    env::set_var("ARGV0", argv0);
    env::set_var("RUNDIR", self_exe_dir);
    env::set_var("RUNSRC", argv0_path);

    let err = Command::new(static_bash).args(&exec_args).exec();
    error_msg(&err.to_string());
    exit(1);
}
