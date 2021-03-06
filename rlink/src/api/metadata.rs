use std::fmt::{Display, Formatter};

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "param")]
pub enum MetadataStorageMode {
    Memory,
    // Zookeeper(String),
    // Etcd {
    //     job_id: String,
    //     endpoints: Vec<String>,
    // },
}

impl MetadataStorageMode {
    pub fn from(
        metadata_storage_mode: &str,
        _metadata_storage_endpoints: &Vec<String>,
        _job_id: &str,
    ) -> Self {
        match metadata_storage_mode.to_ascii_lowercase().as_str() {
            "memory" => MetadataStorageMode::Memory,
            // "etcd" => MetadataStorageMode::Etcd {
            //     job_id: job_id.to_string(),
            //     endpoints: metadata_storage_endpoints.clone(),
            // },
            _ => panic!(format!(
                "Not supported `metadata_storage_mode`={}`",
                metadata_storage_mode
            )),
        }
    }
}
impl Display for MetadataStorageMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MetadataStorageMode::Memory => write!(f, "Memory"),
            // MetadataStorageMode::Zookeeper(path) => write!(f, "Zookeeper{{path={}}}", path),
            // MetadataStorageMode::Etcd { job_id, endpoints } => {
            //     write!(f, "Etcd{{job_id={}, endpoints={:?}}}", job_id, endpoints)
            // }
        }
    }
}
