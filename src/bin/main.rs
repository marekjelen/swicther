use std::env;
use std::process::exit;
use std::fs;
use std::path::Path;
use std::os::unix;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        exit(1);
    }
    
    let path = match std::env::home_dir() {
        Some(pth) => pth.join(format!("{}-{}", ".switcher", args[2])),
        None => exit(1),
    };

    let kube = match std::env::home_dir() {
        Some(pth) => pth.join(".kube"),
        None => exit(1),
    };

    match args[1].as_ref() {
        "init" => init(path.as_path(), kube.as_path()),
        "create" => create(path.as_path()),
        "switch" => switch(path.as_path(), kube.as_path()),
        "destroy" => destroy(path.as_path()),
        "help" => help(),
        _ => help(),
    }
}

fn init(path: &Path, kube: &Path) {
    if kube.exists() && kube.is_dir() {
        fs::rename(kube, path).unwrap();
    }
    switch(path, kube);
}

fn create(path: &Path) {
    fs::create_dir_all(path).unwrap();
}

fn switch(path: &Path, kube: &Path) {
    if kube.exists() && kube.is_dir() {
        fs::remove_file(kube).unwrap();
    }
    unix::fs::symlink(path, kube).unwrap();
}

fn destroy(path: &Path) {
    fs::remove_dir_all(path).unwrap();
}

fn help() {
    println!("./switcher <verb> <profile>"); 
    println!("<verb> = init, create, switch, destroy"); 
    println!("<profile> = profile name"); 
}