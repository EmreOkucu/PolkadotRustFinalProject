use git2::Repository;
use std::env;
use std::process::Command;
use webbrowser;

fn main() {
    // Set the paths and URLs for the node and front-end repositories
    let node_path = "substrate-node-template";
    let node_url = "https://github.com/substrate-developer-hub/substrate-node-template";

    // Clone the node repository
    let _ = Repository::clone(node_url, node_path);

    // Change the working directory to the cloned node repository
    let _ = env::set_current_dir(node_path);

    // Create a new branch in the node repository
    let _ = Command::new("git").args(&["switch", "-c", "my-new-branch"]).output();

    // Build and compile the node with optimizations
    let _ = Command::new("cargo").args(&["build", "--release"]).output();

    // Start the node in developer mode
    let _ = Command::new("./target/release/node-template").args(&["--dev"]).spawn();

    // Print a message indicating the node has started
    println!("Node started in the {} directory on my-new-branch", node_path);

    // Change back to the parent directory
    let _ = env::set_current_dir("..");

    // Set the paths and URLs for the front-end repository
    let front_end_path = "substrate-front-end-template";
    let front_end_url = "https://github.com/substrate-developer-hub/substrate-front-end-template";

    // Clone the front-end repository
    let _ = Repository::clone(front_end_url, front_end_path);

    // Change the working directory to the cloned front-end repository
    let _ = env::set_current_dir(front_end_path);

    // Install front-end dependencies using yarn
    let _ = Command::new("yarn").output();

    // Start the front end server
    let _ = Command::new("yarn").arg("start").output();

    // Print a message indicating the front-end has started
    println!("Front end started in the {} directory", front_end_path);

    // Open the front-end page in the default web browser
    match webbrowser::open("http://localhost:8000") {
        Ok(_) => {} // Success, do nothing
        Err(e) => eprintln!("Error opening front-end page: {}", e),
    }
}
