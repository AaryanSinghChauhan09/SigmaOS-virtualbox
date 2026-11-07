//! virtualbox model — Oracle VirtualBox VM management.

#[derive(Debug, Clone)]
pub struct VM {
    pub name: String,
    pub state: String,
    pub ram: u64,
    pub vcpus: u32,
    pub os_type: String,
    pub storage: String,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub vms: Vec<VM>,
}

impl Default for Model {
    fn default() -> Self { Self { vms: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn create(&mut self, name: &str, os_type: &str, ram: u64, vcpus: u32, storage: &str) -> bool {
        self.vms.push(VM { name: name.into(), state: "PoweredOff".into(), ram, vcpus, os_type: os_type.into(), storage: storage.into() });
        true
    }

    pub fn start(&mut self, name: &str) -> bool {
        if let Some(v) = self.vms.iter_mut().find(|v| v.name == name) { v.state = "Running".into(); true } else { false }
    }

    pub fn list(&self) -> &[VM] { &self.vms }

    pub fn delete(&mut self, name: &str) -> bool {
        if let Some(pos) = self.vms.iter().position(|v| v.name == name) { self.vms.remove(pos); true } else { false }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_start_delete() {
        let mut vb = Model::new();
        assert!(vb.create("vm1", "Ubuntu", 4096, 2, "50G"));
        assert!(vb.start("vm1"));
        assert!(vb.delete("vm1"));
        assert!(vb.list().is_empty());
    }
}
