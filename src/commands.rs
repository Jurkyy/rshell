use std::env;
use std::path::PathBuf;

pub fn cd(args: &[&str]) {
    let new_dir = args.first().cloned().unwrap_or("~");

    let resolved_dir = match new_dir {
        "~" => get_home_directory(),
        path => PathBuf::from(path),
    };

    let cloned_dir = &resolved_dir; // Clone resolved_dir

    if let Err(e) = env::set_current_dir(cloned_dir) {
        eprintln!("Error: {}", e);
    }
}

fn get_home_directory() -> PathBuf {
    if let Some(home_dir) = home_directory() {
        home_dir
    } else {
        eprintln!("Failed to determine the home directory.");
        PathBuf::new()
    }
}

#[cfg(target_os = "windows")]
fn home_directory() -> Option<PathBuf> {
    std::env::var_os("USERPROFILE").map(PathBuf::from)
}

#[cfg(not(target_os = "windows"))]
fn home_directory() -> Option<PathBuf> {
    std::env::var_os("HOME").map(PathBuf::from)
}

pub fn echo(args: &[&str]) {
    let text = args.join(" ");
    println!("{}", text);
}
