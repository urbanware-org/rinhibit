/*
    RInhibit - Prevent accidental execution of certain commands
    Simplified Rust port of the Inhibit project
    Copyright (c) 2024 by Ralf Kilian
    Distributed under the MIT License (https://opensource.org/licenses/MIT)

    GitHub: https://github.com/urbanware-org/rinhibit
    GitLab: https://gitlab.com/urbanware-org/rinhibit
*/

use std::io;
use std::io::Write;
use std::process::Command;
use hostname::get;

fn get_binary_name() -> Option<String> {
    std::env::current_exe()
        .ok()?
        .file_name()?
        .to_str()?
        .to_owned()
        .into()
}

fn main() {
    let version = "1.0.1";
    let usage = "usage: ".to_owned()
                        + &get_binary_name().unwrap()
                        + " [--version] \"COMMAND\"";

    let cmd: String = std::env::args().nth(1).expect(&usage);
    if cmd == "--version" {
        println!("{}", version);
        std::process::exit(0);
    }

    let header = "
####  ##    ##  ##    ##  ####  #######   ####  ########  ########  #######
 ##   ###   ##  ##    ##   ##   ##    ##   ##      ##     ##        ##    ##
 ##   ####  ##  ##    ##   ##   ##    ##   ##      ##     ##        ##    ##
 ##   ## ## ##  ########   ##   #######    ##      ##     ######    ##    ##
 ##   ##  ####  ##    ##   ##   ##    ##   ##      ##     ##        ##    ##
 ##   ##   ###  ##    ##   ##   ##    ##   ##      ##     ##        ##    ##
####  ##    ##  ##    ##  ####  #######   ####     ##     ########  #######";

    println!("{}", header);
    println!();
    println!("Warning! The '{}' command has been inhibited!", cmd);
    println!();
    println!("In order to proceed you have to confirm the process.");
    println!();

    let hostname: String;
    let mut confirm = String::new();

    if let Ok(host) = get() {
        hostname = host.to_string_lossy().to_string();
    } else {
       println!("Failed to get the hostname");
       std::process::exit(1);
    }

    println!("Hostname: {}", hostname);
    print!("Confirm:  ");
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut confirm).expect("No input given");

    println!();
    if hostname == confirm.trim().to_string() {
        println!("Confirmation successful. Proceeding.\n");
        let _ = Command::new(cmd)
            .status()
            .expect("Failed to execute the given command");
    } else {
        println!("Confirmation failed. Process canceled.\n");
        std::process::exit(1);
    }
}

