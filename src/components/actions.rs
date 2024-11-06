use std::{
    fs, path::Path, time::Instant, process::Command,
    io::{ self, Write }
};

use crate::components::{
    constants::AUTO_EXECUTION_COOLDOWN_SECS,
    structs::{ RamMonitor, MemoryAction }
};

impl RamMonitor {
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

    pub fn run_rammap(&mut self, parameter: &str, action_name: &str) {
        if let Err(e) = self.ensure_rammap_exists() {
            self.add_log(format!("Failed to download RAMMap: {}", e), true);
            return;
        }

        self.add_log(format!("Executing: {}...", action_name), false);
        match Command::new("RAMMap64.exe").arg(parameter).spawn() {
            Ok(_) => {
                self.add_log(format!("Successfully executed: {}", action_name), false);
            },
            Err(e) => {
                let error_msg = format!("Failed to execute RAMMap64: {}", e);
                self.add_log(error_msg, true);
            },
        }
    }

    pub fn check_auto_execution(&mut self, current_percentage: f32) {
        if current_percentage >= self.auto_threshold {
            if self.last_auto_execution.map_or(true, |time| time.elapsed().as_secs() > AUTO_EXECUTION_COOLDOWN_SECS) {
                let action = match self.auto_action.as_str() {
                    "Empty Working Sets" => MemoryAction::EmptyWorkingSets,
                    "Empty System Working Sets" => MemoryAction::EmptySystemWorkingSets,
                    "Empty Modified Page Lists" => MemoryAction::EmptyModifiedPageLists,
                    "Empty Standby List" => MemoryAction::EmptyStandbyList,
                    "Empty Priority 0 Standby List" => MemoryAction::EmptyPriorityZeroStandbyList,
                    _ => MemoryAction::EmptyWorkingSets,
                };
                
                self.run_rammap(action.parameter(), action.display_name());
                self.last_auto_execution = Some(Instant::now());
            }
        }
    }

    pub fn execute_memory_action(&mut self, action: MemoryAction) {
        self.run_rammap(action.parameter(), action.display_name());
    }
}
