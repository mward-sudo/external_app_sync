use anyhow::Result;
use external_app_sync::{config::Config, create_alias, get_applications_dir, is_app_bundle, remove_alias};
use notify::{Event, EventKind, RecursiveMode, Watcher};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::error;

struct AppMonitor {
    config: Arc<Mutex<Config>>,
}

impl AppMonitor {
    fn new(config: Config) -> Self {
        Self {
            config: Arc::new(Mutex::new(config)),
        }
    }

    async fn handle_event(&self, event: Event) -> Result<()> {
        let _config = self.config.lock().await;
        
        match event.kind {
            EventKind::Create(_) | EventKind::Modify(_) => {
                for path in event.paths {
                    if is_app_bundle(&path) {
                        let app_name = path.file_name().unwrap().to_string_lossy();
                        let dest = get_applications_dir().join(&*app_name);
                        
                        if !dest.exists() {
                            if let Err(e) = create_alias(&path, &dest) {
                                error!("Failed to create alias for {}: {}", app_name, e);
                            }
                        }
                    }
                }
            }
            EventKind::Remove(_) => {
                for path in event.paths {
                    if is_app_bundle(&path) {
                        let app_name = path.file_name().unwrap().to_string_lossy();
                        let alias_path = get_applications_dir().join(&*app_name);
                        
                        if let Err(e) = remove_alias(&alias_path) {
                            error!("Failed to remove alias for {}: {}", app_name, e);
                        }
                    }
                }
            }
            _ => {}
        }
        
        Ok(())
    }

    async fn sync_initial_state(&self) -> Result<()> {
        let config = self.config.lock().await;
        let external_dir = &config.external_apps_path;
        
        // Remove stale aliases
        for entry in std::fs::read_dir(get_applications_dir())? {
            let entry = entry?;
            let path = entry.path();
            
            if is_app_bundle(&path) {
                // Check if this is an alias and if its target is in our external directory
                let app_name = path.file_name().unwrap();
                let external_app = external_dir.join(app_name);
                
                if !external_app.exists() {
                    if let Err(e) = remove_alias(&path) {
                        error!("Failed to remove stale alias {}: {}", path.display(), e);
                    }
                }
            }
        }
        
        // Create missing aliases
        for entry in std::fs::read_dir(external_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if is_app_bundle(&path) {
                let app_name = path.file_name().unwrap();
                let alias_path = get_applications_dir().join(app_name);
                
                if !alias_path.exists() {
                    if let Err(e) = create_alias(&path, &alias_path) {
                        error!("Failed to create alias for {}: {}", path.display(), e);
                    }
                }
            }
        }
        
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Load config
    let config = Config::load().expect("Failed to load config");
    let monitor = AppMonitor::new(config);
    
    // Perform initial sync
    monitor.sync_initial_state().await?;
    
    // Set up file system watcher
    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher = notify::recommended_watcher(tx)?;
    
    let config = monitor.config.lock().await;
    watcher.watch(&config.external_apps_path, RecursiveMode::Recursive)?;
    drop(config);
    
    // Handle events
    for res in rx {
        match res {
            Ok(event) => {
                if let Err(e) = monitor.handle_event(event).await {
                    error!("Error handling event: {}", e);
                }
            }
            Err(e) => error!("Watch error: {}", e),
        }
    }
    
    Ok(())
}
