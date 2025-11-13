use anyhow::{anyhow, Result};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

use crate::roles::Role;
use crate::scripts::Script;

#[derive(Debug)]
pub struct ValidationReport {
    pub valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub scripts_count: usize,
    pub roles_count: usize,
}

impl ValidationReport {
    pub fn new() -> Self {
        Self {
            valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
            scripts_count: 0,
            roles_count: 0,
        }
    }

    pub fn add_error(&mut self, msg: String) {
        self.valid = false;
        self.errors.push(msg);
    }

    pub fn add_warning(&mut self, msg: String) {
        self.warnings.push(msg);
    }

    pub fn print(&self) {
        println!("\n=== Validation Report ===\n");
        println!("Scripts found: {}", self.scripts_count);
        println!("Roles found: {}", self.roles_count);
        println!();

        if !self.errors.is_empty() {
            println!("❌ Errors ({})", self.errors.len());
            for (i, err) in self.errors.iter().enumerate() {
                println!("  {}. {}", i + 1, err);
            }
            println!();
        }

        if !self.warnings.is_empty() {
            println!("⚠️  Warnings ({})", self.warnings.len());
            for (i, warn) in self.warnings.iter().enumerate() {
                println!("  {}. {}", i + 1, warn);
            }
            println!();
        }

        if self.valid {
            println!("✅ All validations passed!");
        } else {
            println!("❌ Validation failed with {} errors", self.errors.len());
        }
    }
}

impl Default for ValidationReport {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ContentValidator {
    scripts: HashMap<String, Script>,
    roles: HashMap<String, Role>,
}

impl ContentValidator {
    pub fn new() -> Self {
        Self {
            scripts: HashMap::new(),
            roles: HashMap::new(),
        }
    }

    /// Load all scripts from a directory
    pub fn load_scripts(&mut self, dir: &Path) -> Result<()> {
        if !dir.exists() {
            return Err(anyhow!("Scripts directory not found: {:?}", dir));
        }

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
                // Skip files starting with underscore (like _role.yaml)
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if name.starts_with('_') {
                        continue;
                    }
                }

                match self.load_script(&path) {
                    Ok(script) => {
                        self.scripts.insert(script.id.clone(), script);
                    }
                    Err(e) => {
                        return Err(anyhow!("Failed to load script {:?}: {}", path, e));
                    }
                }
            }
        }

        Ok(())
    }

    fn load_script(&self, path: &Path) -> Result<Script> {
        let content = fs::read_to_string(path)?;
        let script: Script = serde_yaml::from_str(&content)?;
        Ok(script)
    }

    /// Load all roles recursively from a directory
    pub fn load_roles_recursive(&mut self, dir: &Path) -> Result<()> {
        if !dir.exists() {
            return Err(anyhow!("Roles directory not found: {:?}", dir));
        }

        self.scan_roles_dir(dir)?;
        Ok(())
    }

    fn scan_roles_dir(&mut self, dir: &Path) -> Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                // Recursively scan subdirectories
                self.scan_roles_dir(&path)?;
            } else if path.file_name().and_then(|n| n.to_str()) == Some("_role.yaml") {
                // Load role manifest
                match self.load_role(&path) {
                    Ok(role) => {
                        self.roles.insert(role.id.clone(), role);
                    }
                    Err(e) => {
                        return Err(anyhow!("Failed to load role {:?}: {}", path, e));
                    }
                }
            }
        }

        Ok(())
    }

    fn load_role(&self, path: &Path) -> Result<Role> {
        let content = fs::read_to_string(path)?;
        let role: Role = serde_yaml::from_str(&content)?;
        Ok(role)
    }

    /// Validate all loaded content
    pub fn validate(&self) -> ValidationReport {
        let mut report = ValidationReport::new();
        report.scripts_count = self.scripts.len();
        report.roles_count = self.roles.len();

        // Check for duplicate IDs
        self.check_duplicate_ids(&mut report);

        // Validate each script
        for script in self.scripts.values() {
            self.validate_script(script, &mut report);
        }

        // Validate each role
        for role in self.roles.values() {
            self.validate_role(role, &mut report);
        }

        // Cross-validate roles and scripts
        self.validate_role_scenario_links(&mut report);

        report
    }

    fn check_duplicate_ids(&self, report: &mut ValidationReport) {
        let mut script_ids: HashSet<String> = HashSet::new();

        for script in self.scripts.values() {
            if !script_ids.insert(script.id.clone()) {
                report.add_error(format!("Duplicate script ID: {}", script.id));
            }
        }

        let mut role_ids: HashSet<String> = HashSet::new();

        for role in self.roles.values() {
            if !role_ids.insert(role.id.clone()) {
                report.add_error(format!("Duplicate role ID: {}", role.id));
            }
        }
    }

    fn validate_script(&self, script: &Script, report: &mut ValidationReport) {
        // Check required fields
        if script.id.is_empty() {
            report.add_error("Script has empty ID".to_string());
        }

        if script.title.is_empty() {
            report.add_error(format!("Script '{}' has empty title", script.id));
        }

        if script.steps.is_empty() {
            report.add_error(format!("Script '{}' has no steps", script.id));
        }

        // Check steps
        for (i, step) in script.steps.iter().enumerate() {
            if let Some(content) = &step.content {
                if content.is_empty() {
                    report.add_warning(format!(
                        "Script '{}' step {} has empty content",
                        script.id, i
                    ));
                }
            }

            if step.prompt.is_empty() {
                report.add_warning(format!(
                    "Script '{}' step {} has empty prompt",
                    script.id, i
                ));
            }
        }
    }

    fn validate_role(&self, role: &Role, report: &mut ValidationReport) {
        // Check required fields
        if role.id.is_empty() {
            report.add_error("Role has empty ID".to_string());
        }

        if role.title.is_empty() {
            report.add_error(format!("Role '{}' has empty title", role.id));
        }

        if role.description.is_empty() {
            report.add_warning(format!("Role '{}' has empty description", role.id));
        }

        if role.scenario_ids.is_empty() {
            report.add_error(format!("Role '{}' has no scenarios", role.id));
        }
    }

    fn validate_role_scenario_links(&self, report: &mut ValidationReport) {
        for role in self.roles.values() {
            for scenario_id in &role.scenario_ids {
                // Check if scenario file exists (scenario_id is filename)
                let scenario_exists = self.scripts.values().any(|s| {
                    // Match by filename or ID
                    s.id == *scenario_id || scenario_id.contains(&s.id)
                });

                if !scenario_exists {
                    report.add_warning(format!(
                        "Role '{}' references scenario '{}' which was not found",
                        role.id, scenario_id
                    ));
                }
            }
        }
    }
}

impl Default for ContentValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_report_creation() {
        let report = ValidationReport::new();
        assert!(report.valid);
        assert_eq!(report.errors.len(), 0);
        assert_eq!(report.warnings.len(), 0);
    }

    #[test]
    fn test_validation_report_errors() {
        let mut report = ValidationReport::new();
        assert!(report.valid);

        report.add_error("Test error".to_string());
        assert!(!report.valid);
        assert_eq!(report.errors.len(), 1);
    }

    #[test]
    fn test_validation_report_warnings() {
        let mut report = ValidationReport::new();
        report.add_warning("Test warning".to_string());

        assert!(report.valid);
        assert_eq!(report.warnings.len(), 1);
    }
}
