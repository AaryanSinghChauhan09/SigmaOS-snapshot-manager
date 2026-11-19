// SigmaOS Snapshot Manager CLI
use sigmaos_snapshot_manager::{SnapshotManager, PreUpdateSnapshotFactory, ManualSnapshotFactory, SnapshotFactory};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        println!("Usage: sigmaos-snapshot-manager <type> <description>");
        println!("Types: pre-update, manual");
        return;
    }
    
    let snapshot_type = &args[1];
    let description = &args[2];
    let mut manager = SnapshotManager::new();
    
    let factory: Box<dyn SnapshotFactory> = match snapshot_type.as_str() {
        "pre-update" => Box::new(PreUpdateSnapshotFactory),
        "manual" => Box::new(ManualSnapshotFactory),
        _ => {
            eprintln!("Unknown snapshot type: {}", snapshot_type);
            return;
        }
    };
    
    let command = factory.create_snapshot(description, "/btrfs");
    match manager.execute_command(command) {
        Ok(snapshot) => println!("Snapshot created: {}", snapshot.id),
        Err(e) => eprintln!("Failed to create snapshot: {}", e),
    }
}
