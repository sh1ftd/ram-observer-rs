use std::{
    io::{ self, Write },
    fs, path::Path, time::Instant, process::Command
};

use crate::components::{
    structs::RamMonitor,
    memory_management::Commands,
    constants::AUTO_EXECUTION_COOLDOWN_SECS
};

impl RamMonitor {
    /// Downloads and extracts RAMMap64.exe if it doesn't exist in the current directory
    /// 
    /// # Arguments
    /// * `self` - Mutable reference to RamMonitor instance
    /// 
    /// # Returns
    /// * `io::Result<()>` - Success or error during download/extraction
    /// 
    /// # Process
    /// 1. Checks if RAMMap64.exe exists
    /// 2. If not, downloads RAMMap.zip from Sysinternals
    /// 3. Extracts RAMMap64.exe from the zip
    /// 4. Cleans up temporary files
    pub fn ensure_rammap_exists(&mut self) -> io::Result<()> {
        if !Path::new("RAMMap64.exe").exists() {
            self.add_log("RAMMap64.exe not found. Downloading...".to_string(), false);
            
            // Download the zip file
            let response = reqwest::blocking::get("https://download.sysinternals.com/files/RAMMap.zip")
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
            
            let bytes = response.bytes()
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
            
            // Save zip file temporarily
            let mut temp_file = fs::File::create("rammap_temp.zip")?;
            temp_file.write_all(&bytes)?;
            
            // Extract RAMMap64.exe from the zip
            let file = fs::File::open("rammap_temp.zip")?;
            let mut archive = zip::ZipArchive::new(file)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
            
            for i in 0..archive.len() {
                let mut file = archive.by_index(i)
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
                
                if file.name() == "RAMMap64.exe" {
                    let mut outfile = fs::File::create("RAMMap64.exe")?;
                    io::copy(&mut file, &mut outfile)?;
                    break;
                }
            }
            
            // Clean up the temporary zip file
            fs::remove_file("rammap_temp.zip")?;
            self.add_log("Successfully downloaded RAMMap64.exe".to_string(), false);
        }
        Ok(())
    }

    /// Checks if automatic RAM management should be executed based on current memory usage
    /// 
    /// # Arguments
    /// * `self` - Mutable reference to RamMonitor instance
    /// * `current_percentage` - Current RAM usage percentage
    /// 
    /// # Behavior
    /// * Executes the configured auto-action if:
    ///   1. Current RAM usage exceeds auto_threshold
    ///   2. Enough time has passed since last auto-execution
    pub fn check_auto_execution(&mut self, current_percentage: f32) {
        if current_percentage >= self.auto_threshold && self.last_auto_execution.map_or(true, |time| time.elapsed().as_secs() > AUTO_EXECUTION_COOLDOWN_SECS) {
            let action = match self.auto_action.as_str() {
                "Empty Working Sets" => Commands::EmptyWorkingSets,
                "Empty System Working Sets" => Commands::EmptySystemWorkingSets,
                "Empty Modified Page Lists" => Commands::EmptyModifiedPageLists,
                "Empty Standby List" => Commands::EmptyStandbyList,
                "Empty Priority 0 Standby List" => Commands::EmptyPriorityZeroStandbyList,
                _ => Commands::EmptyWorkingSets,
            };
            
            self.run_rammap(action);
            self.last_auto_execution = Some(Instant::now());
        }
    }

    /// Executes a RAM management command using RAMMap64.exe
    /// 
    /// # Arguments
    /// * `self` - Mutable reference to RamMonitor instance
    /// * `action` - The RAM management command to execute
    /// 
    /// # Process
    /// 1. Ensures RAMMap64.exe exists
    /// 2. Executes the specified command
    /// 3. Logs the result (success or failure)
    /// 
    /// # Note
    /// This function will attempt to download RAMMap64.exe if it's not found
    pub fn run_rammap(&mut self, action: Commands) {
        if let Err(e) = self.ensure_rammap_exists() {
            self.add_log(format!("Failed to download RAMMap: {}", e), true);
            return;
        }

        let display_name = action.display_name();
        self.add_log(format!("Executing: {}...", display_name), false);
        match Command::new("RAMMap64.exe").arg(action.parameter()).spawn() {
            Ok(_) => {
                self.add_log(format!("Successfully executed: {}", display_name), false);
            },
            Err(e) => {
                let error_msg = format!("Failed to execute RAMMap64: {}", e);
                self.add_log(error_msg, true);
            },
        }
    }
}
