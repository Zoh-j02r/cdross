use clap::Parser;
use std::io;
use std::io::Write;
use std::env;
use std::process;
use std::collections::HashMap;
use colored::Colorize;
use std::process::Command;

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
    let mut file = std::fs::File::create("/tmp/Dockerfile").expect("create failed");
    println!("{}"," # Select distro do be used:".blue());
    {
        let input = input_user();
        if vec!["melodic", "noetic", "foxy"].iter().any(|e| input.contains(e)) {
            _distro_ver = input;
            _ros_ver = (if _distro_ver == "foxy" {"2"} else {""}).to_string();
            //format!("FROM ros:{}-ros-base",_distro_ver.trim());
            file.write_all(format!("FROM ros:{}-ros-base\n",_distro_ver.trim()).as_bytes()).expect("something went wrong");
        } else {
            std::fs::remove_file("/tmp/Dockerfile").expect("could not remove file");
            println!("{}: Distro not found or does not exist","ERROR".red());
            process::exit(1);
        }
    }
    println!("{}"," # Set packages to be added to container image:".blue());
    println!("Currently these are the supported packages:");
    println!(" gazebo rviz ");
    {
        let input = input_user();
        let asda = package_sel(_distro_ver,_ros_ver,input);
        file.write_all(
            format!("RUN apt update && apt install libgl1-mesa-glx libgl1-mesa-dri \\
            {} \\
            sudo\n",asda.trim())
            .as_bytes())
            .expect("something went wrong");
        //file.write_all(format!("{:?}",asda).as_bytes()).expect("something went wrong");
        //file.write_all(format!("  sudo\n").as_bytes()).expect("something went wrong");
    }
    println!("{}"," # Set the username to be used by container:".blue());
    {
        let input = input_user();
        file.write_all(format!("RUN useradd --create-home --home-dir /home/cdross-dir \\ 
            --shell /bin/bash --user-group --groups sudo {user} && \\ 
	    echo {user}:{user} | chpasswd && \\ 
	    echo {user} ALL=(ALL) NOPASSWD:ALL",user=input.trim()).as_bytes()).expect("something went wrong")

        //println!("RUN useradd --create-home --home-dir /home/cdross-dir \\ 
        //    --shell /bin/bash --user-group --groups sudo {user} && \\ 
	    //echo {user}:{user} | chpasswd && \\ 
	    //echo {user} ALL=(ALL) NOPASSWD:ALL",user=input.trim()); 
    }
}

fn run() {
    let xlib = env::var("DISPLAY").unwrap();
        let wayl = env::var("WAYLAND_DISPLAY").unwrap_or("none".to_string());
    let xlib = "DISPLAY=".to_owned() + &xlib;
    let wayl = "WAYLAND_DISPLAY=".to_owned() + &wayl;
    println!("{}",wayl);
    println!("{}",xlib);
    println!("{}","Starting cdross container".blue());
    Command::new("docker")
        .arg("run")
        .arg("-d")
        .arg("--name")
        .arg("cdross")
        .arg("--rm")
        .arg("-it")
        .arg("-v")
        .arg("/tmp/.X11-unix:/tmp/.X11-unix")
        .arg("-e")
        .arg(&xlib)
        .arg("-e")
        .arg(&wayl) 
        .arg("--device=/dev/dri/card0:/dev/dri/card0")
        .arg("cdross")
        .output()
        .expect("could not start docker make sure your user is in docker group");
}

fn exec() {
    Command::new("docker")
        .arg("exec")
        .arg("--user")
        .arg("cross")
        .arg("-it")
        .arg("cdross")
        .arg("bash")
        .arg("-c")
        .arg("source /opt/ros/foxy/setup.sh && rviz2")
        .spawn()
        .expect("could not send command");
}


fn input_user() -> String {
    let mut _choice = String::new();
    io::stdin().read_line(&mut _choice).expect("Could not access value");
    _choice.to_owned()
}

fn package_sel(distro: String,ver: String,pak: String) -> String {
    let mut ppk = String::new();
    let values:Vec<&str> = pak.split(" ").collect();
    let mut packages = HashMap::new();
    packages.insert("gazebo",format!("ros-{}-gazebo-ros-pkgs",distro.trim()));
    packages.insert("rviz",format!("ros-{}-rviz{}",distro.trim(),ver.trim()));
    for i in 0..values.len() {
        let aux = values[i].trim();
        if packages.contains_key(aux) {
            ppk += &format!("{} ",packages[aux]).to_string();
        }
    }
    ppk
}

fn main() {
    let _cli = Cli::parse();
    if _cli.init || _cli.run || _cli.exec {
        if _cli.init {
            init()
        }
        if _cli.run {
            run()
        }
        if _cli.exec {
            exec()
        }
    } else {
        println! ("ERROR: a argument must be privided");
        println! ("To see possible arguments use '--help'");
        process::exit(1);
    }
}
