// SigmaOS Snapshot Manager - BTRFS snapshot lifecycle management
// OOPS Principles: Command, Factory patterns
// Design Patterns: Command, Factory

/// Snapshot information
#[derive(Debug, Clone)]
pub struct Snapshot {
    pub id: String,
    pub description: String,
    pub timestamp: u64,
    pub path: String,
}

/// Command Pattern - Snapshot operations as commands
pub trait SnapshotCommand {
    fn execute(&self) -> Result<Snapshot, String>;
    fn undo(&self) -> Result<(), String>;
}

/// Create snapshot command
pub struct CreateSnapshotCommand {
    description: String,
    path: String,
}

impl CreateSnapshotCommand {
    pub fn new(description: &str, path: &str) -> Self {
        Self {
            description: description.to_string(),
            path: path.to_string(),
        }
    }
}

impl SnapshotCommand for CreateSnapshotCommand {
    fn execute(&self) -> Result<Snapshot, String> {
        println!("Creating snapshot: {} at {}", self.description, self.path);
        Ok(Snapshot {
            id: "snap-001".to_string(),
            description: self.description.clone(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            path: self.path.clone(),
        })
    }
    
    fn undo(&self) -> Result<(), String> {
        println!("Undoing snapshot creation");
        Ok(())
    }
}

/// Delete snapshot command
pub struct DeleteSnapshotCommand {
    snapshot_id: String,
}

impl DeleteSnapshotCommand {
    pub fn new(snapshot_id: &str) -> Self {
        Self {
            snapshot_id: snapshot_id.to_string(),
        }
    }
}

impl SnapshotCommand for DeleteSnapshotCommand {
    fn execute(&self) -> Result<Snapshot, String> {
        println!("Deleting snapshot: {}", self.snapshot_id);
        Ok(Snapshot {
            id: self.snapshot_id.clone(),
            description: "Deleted".to_string(),
            timestamp: 0,
            path: "".to_string(),
        })
    }
    
    fn undo(&self) -> Result<(), String> {
        println!("Undoing snapshot deletion");
        Ok(())
    }
}

/// Factory Pattern - Create different snapshot types
pub trait SnapshotFactory {
    fn create_snapshot(&self, description: &str, path: &str) -> Box<dyn SnapshotCommand>;
}

/// Pre-update snapshot factory
pub struct PreUpdateSnapshotFactory;

impl SnapshotFactory for PreUpdateSnapshotFactory {
    fn create_snapshot(&self, description: &str, path: &str) -> Box<dyn SnapshotCommand> {
        Box::new(CreateSnapshotCommand::new(&format!("Pre-update: {}", description), path))
    }
}

/// Manual snapshot factory
pub struct ManualSnapshotFactory;

impl SnapshotFactory for ManualSnapshotFactory {
    fn create_snapshot(&self, description: &str, path: &str) -> Box<dyn SnapshotCommand> {
        Box::new(CreateSnapshotCommand::new(&format!("Manual: {}", description), path))
    }
}

/// Snapshot manager
pub struct SnapshotManager {
    snapshots: Vec<Snapshot>,
}

impl SnapshotManager {
    pub fn new() -> Self {
        Self {
            snapshots: vec![],
        }
    }
    
    pub fn execute_command(&mut self, command: Box<dyn SnapshotCommand>) -> Result<Snapshot, String> {
        let snapshot = command.execute()?;
        self.snapshots.push(snapshot.clone());
        Ok(snapshot)
    }
    
    pub fn list_snapshots(&self) -> &Vec<Snapshot> {
        &self.snapshots
    }
    
    pub fn cleanup_old_snapshots(&mut self, retention: usize) -> Result<(), String> {
        if self.snapshots.len() > retention {
            self.snapshots.truncate(retention);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_command_pattern() {
        let command = CreateSnapshotCommand::new("Test", "/btrfs");
        let result = command.execute();
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_factory_pattern() {
        let factory = PreUpdateSnapshotFactory;
        let command = factory.create_snapshot("Update", "/btrfs");
        let result = command.execute();
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_snapshot_manager() {
        let mut manager = SnapshotManager::new();
        let command = CreateSnapshotCommand::new("Test", "/btrfs");
        let result = manager.execute_command(Box::new(command));
        assert!(result.is_ok());
        assert_eq!(manager.list_snapshots().len(), 1);
    }
    
    #[test]
    fn test_cleanup() {
        let mut manager = SnapshotManager::new();
        for i in 0..5 {
            let command = CreateSnapshotCommand::new(&format!("Snap {}", i), "/btrfs");
            let _ = manager.execute_command(Box::new(command));
        }
        manager.cleanup_old_snapshots(3).unwrap();
        assert_eq!(manager.list_snapshots().len(), 3);
    }
}
