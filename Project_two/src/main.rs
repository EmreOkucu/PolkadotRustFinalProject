use std::fs;
use std::process::Command;

fn main() {
    let node_binary_path = "node-template";

    // Define directories for Alice and Bob within the specified directory
    let alice_path = "C:\\Users\\Emre\\Desktop\\Project_two\\alice";
    let bob_path = "C:\\Users\\Emre\\Desktop\\Project_two\\bob";
    
    // Create directories for Alice and Bob if they don't exist
    fs::create_dir_all(alice_path).expect("Failed to create directory for Alice");
    fs::create_dir_all(bob_path).expect("Failed to create directory for Bob");

    // Purge old chain data for Alice
    Command::new(node_binary_path)
        .arg("purge-chain")
        .arg("--base-path")
        .arg(alice_path)
        .arg("--chain")
        .arg("local")
        .arg("-y")
        .output()
        .expect("Failed to purge old chain data for Alice");

    // Start Alice's node
    Command::new(node_binary_path)
        .arg("--base-path")
        .arg(alice_path)
        .arg("--chain")
        .arg("local")
        .arg("--alice")
        .arg("--port")
        .arg("30333")
        .arg("--ws-port")
        .arg("9945")
        .arg("--rpc-port")
        .arg("9933")
        .arg("--telemetry-url")
        .arg("wss://telemetry.polkadot.io/submit/ 0")
        .arg("--validator")
        .spawn()
        .expect("Failed to start the node for Alice");

    println!("Node started for Alice");

    // Purge old chain data for Bob
    Command::new(node_binary_path)
        .arg("purge-chain")
        .arg("--base-path")
        .arg(bob_path)
        .arg("--chain")
        .arg("local")
        .arg("-y")
        .output()
        .expect("Failed to purge old chain data for Bob");

    // Start Bob's node
    Command::new(node_binary_path)
        .arg("--base-path")
        .arg(bob_path)
        .arg("--chain")
        .arg("local")
        .arg("--bob")
        .arg("--port")
        .arg("30334")
        .arg("--ws-port")
        .arg("9946")
        .arg("--rpc-port")
        .arg("9934")
        .arg("--telemetry-url")
        .arg("wss://telemetry.polkadot.io/submit/ 0")
        .arg("--validator")
        .spawn()
        .expect("Failed to start the node for Bob");

    println!("Node started for Bob");
}
