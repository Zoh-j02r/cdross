use clap::Parser;
use std::io;
use std::process;
use std::collections::HashMap;
use colored::Colorize;
//use std::process::Command;

#[derive(Parser)]
struct Cli {
    /// Generate container file and build it.
    #[arg(short,long)]
    init: bool,
    /// Start running cdross container.
    #[arg(short,long)]
    run: bool,
    /// Run a command inside a cdross.
    #[arg(short,long)]
    exec: bool,
}

fn init() {
    let mut _distro_ver = String::new(); 
    let mut _ros_ver = String::new();
    println!("{}"," # Select distro do be used:".blue());
    {
        let input = input_user();
        if vec!["melodic", "noetic", "foxy"].iter().any(|e| input.contains(e)) {
            _distro_ver = input;
            _ros_ver = (if _distro_ver == "foxy" {"2"} else {""}).to_string();
            println!("FROM ros:{}-ros-base",_distro_ver.trim());
        } else {
            println!("{}: Distro not found or does not exist","ERROR".red());
            process::exit(1);
        }
    }
    println!("{}"," # Set packages to be added to container image:".blue());
    println!("Currently these are the supported packages:");
    println!(" gazebo rviz ");
    {
        let input = input_user();
        package_selection(_distro_ver,_ros_ver,input);
    }
    //    Command::new("echo")
    //    .arg("pogger")
    //    .spawn()
    //    .expect("could not pogger");
}

fn input_user() -> String {
    let mut _choice = String::new();
    io::stdin().read_line(&mut _choice).expect("Could not access value");
    _choice.to_owned()
}

fn package_selection(distro: String,ver: String,pak: String) {
    let values:Vec<&str> = pak.split(" ").collect();
    let mut packages = HashMap::new();
    packages.insert("gazebo",format!("ros-{}-gazebo-ros-pkgs",distro.trim()));
    packages.insert("rviz",format!("ros-{}-rviz{}",distro.trim(),ver.trim()));
    for i in 0..values.len() {
        let aux = values[i].trim();
        if packages.contains_key(aux) {
            println!("{}",packages[aux]);
        }
    }
}

fn main() {
    let _cli = Cli::parse();
    if _cli.init || _cli.run || _cli.exec {
        if _cli.init {
            init()
        }
        if _cli.run {
            println!("running");
        }
        if _cli.exec {
            println!("executing");
        }
    } else {
        println! ("ERROR: a argument must be privided");
        println! ("To see possible arguments use '--help'");
        process::exit(1);
    }
}
