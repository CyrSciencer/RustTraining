struct EngineVFS {
    path: std::path::PathBuf,
}

impl EngineVFS {
    pub fn new(base_path: &str) -> Self {
        return EngineVFS {
            path: std::path::PathBuf::from(base_path),
        };
    }
    pub fn resolve_vfs_uri(&self, uri: &str) -> Result<std::path::PathBuf, &'static str> {
        if uri.contains("../"){
            return Err("VFS Traversal Security Infraction");
        };
        let cleaned_path: String = uri.replace("://", "/");
        let full_path = self.path.join(cleaned_path);
        return Ok(full_path);
    }
}

pub fn main10() {
    
}
