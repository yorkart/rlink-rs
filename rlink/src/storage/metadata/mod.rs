use crate::runtime::{JobDescriptor, TaskManagerStatus};
// use crate::storage::metadata::etcd_metadata_storage::EtcdMetadataStorage;
use crate::storage::metadata::mem_metadata_storage::MemoryMetadataStorage;
use std::error::Error;
use std::fmt::Debug;

// pub mod etcd_metadata_storage;
pub mod mem_metadata_storage;

pub mod metadata_loader;
use crate::api::metadata::MetadataStorageMode;
pub use metadata_loader::MetadataLoader;

pub trait MetadataStorage: Debug {
    fn save_job_descriptor(
        &mut self,
        metadata: JobDescriptor,
    ) -> Result<(), Box<dyn Error + Send + Sync>>;
    fn delete_job_descriptor(&mut self) -> Result<(), Box<dyn Error + Send + Sync>>;
    fn read_job_descriptor(&self) -> Result<JobDescriptor, Box<dyn Error + Send + Sync>>;
    fn update_job_status(
        &self,
        job_manager_status: TaskManagerStatus,
    ) -> Result<(), Box<dyn Error + Send + Sync>>;
    fn update_task_status(
        &self,
        task_manager_id: &str,
        task_manager_address: &str,
        task_manager_status: TaskManagerStatus,
        metrics_address: &str,
    ) -> Result<(), Box<dyn Error + Send + Sync>>;
}

#[derive(Debug)]
pub enum MetadataStorageWrap {
    MemoryMetadataStorage(MemoryMetadataStorage),
    // EtcdMetadataStorage(EtcdMetadataStorage),
}

impl MetadataStorageWrap {
    pub fn new(mode: &MetadataStorageMode) -> Self {
        match mode {
            MetadataStorageMode::Memory => {
                let storage = MemoryMetadataStorage::new();
                MetadataStorageWrap::MemoryMetadataStorage(storage)
            }
            // MetadataStorageMode::Etcd { job_id, endpoints } => {
            //     let storage = EtcdMetadataStorage::new(job_id.clone(), endpoints.clone());
            //     MetadataStorageWrap::EtcdMetadataStorage(storage)
            // }
            // _ => panic!("Unsupported MetadataStorageMode"),
        }
    }
}

impl MetadataStorage for MetadataStorageWrap {
    fn save_job_descriptor(
        &mut self,
        metadata: JobDescriptor,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        match self {
            MetadataStorageWrap::MemoryMetadataStorage(storage) => {
                storage.save_job_descriptor(metadata)
            } // MetadataStorageWrap::EtcdMetadataStorage(storage) => {
              //     storage.save_job_descriptor(metadata)
              // }
        }
    }

    fn delete_job_descriptor(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        match self {
            MetadataStorageWrap::MemoryMetadataStorage(storage) => storage.delete_job_descriptor(),
            // MetadataStorageWrap::EtcdMetadataStorage(storage) => storage.delete_job_descriptor(),
        }
    }

    fn read_job_descriptor(&self) -> Result<JobDescriptor, Box<dyn Error + Send + Sync>> {
        match self {
            MetadataStorageWrap::MemoryMetadataStorage(storage) => storage.read_job_descriptor(),
            // MetadataStorageWrap::EtcdMetadataStorage(storage) => storage.read_job_descriptor(),
        }
    }

    fn update_job_status(
        &self,
        job_manager_status: TaskManagerStatus,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        match self {
            MetadataStorageWrap::MemoryMetadataStorage(storage) => {
                storage.update_job_status(job_manager_status)
            } // MetadataStorageWrap::EtcdMetadataStorage(storage) => {
              //     storage.update_job_status(job_manager_status)
              // }
        }
    }

    fn update_task_status(
        &self,
        task_manager_id: &str,
        task_manager_address: &str,
        task_manager_status: TaskManagerStatus,
        metrics_address: &str,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        match self {
            MetadataStorageWrap::MemoryMetadataStorage(storage) => storage.update_task_status(
                task_manager_id,
                task_manager_address,
                task_manager_status,
                metrics_address,
            ),
            // MetadataStorageWrap::EtcdMetadataStorage(storage) => storage.update_task_status(
            //     task_manager_id,
            //     task_manager_address,
            //     task_manager_status,
            //     metrics_address,
            // ),
        }
    }
}

pub(crate) fn loop_read_job_descriptor(metadata_storage: &MetadataStorageWrap) -> JobDescriptor {
    loop_fn!(
        metadata_storage.read_job_descriptor(),
        std::time::Duration::from_secs(2)
    )
}

pub(crate) fn loop_save_job_descriptor(
    metadata_storage: &mut MetadataStorageWrap,
    job_descriptor: JobDescriptor,
) {
    loop_fn!(
        metadata_storage.save_job_descriptor(job_descriptor.clone()),
        std::time::Duration::from_secs(2)
    );
}

pub(crate) fn loop_delete_job_descriptor(metadata_storage: &mut MetadataStorageWrap) {
    loop_fn!(
        metadata_storage.delete_job_descriptor(),
        std::time::Duration::from_secs(2)
    );
}

pub(crate) fn loop_update_job_status(
    metadata_storage: &mut MetadataStorageWrap,
    job_manager_status: TaskManagerStatus,
) {
    loop_fn!(
        metadata_storage.update_job_status(job_manager_status.clone()),
        std::time::Duration::from_secs(2)
    );
}
