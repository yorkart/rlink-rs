use crate::api::cluster::TaskResourceInfo;
use crate::api::env::{StreamExecutionEnvironment, StreamJob};
use crate::resource::local::LocalResourceManager;
use crate::resource::standalone::StandaloneResourceManager;
use crate::resource::yarn::YarnResourceManager;
use crate::runtime::context::Context;
use crate::runtime::{ClusterMode, JobDescriptor};
use std::fmt::Debug;

pub mod local;
pub mod standalone;
pub mod yarn;

pub struct Resource {
    memory: u32,
    cpu_cores: u32,
}

impl Resource {
    pub fn new(memory: u32, cpu_cores: u32) -> Self {
        Resource { memory, cpu_cores }
    }
}

pub(crate) trait ResourceManager
where
    Self: Debug,
{
    fn prepare(&mut self, context: &Context, job_descriptor: &JobDescriptor);

    /// worker resource allocate
    /// Return a resource location.
    fn worker_allocate<S>(
        &self,
        stream_job: &S,
        stream_env: &StreamExecutionEnvironment,
    ) -> anyhow::Result<Vec<TaskResourceInfo>>
    where
        S: StreamJob + 'static;

    fn stop_workers(&self, task_ids: Vec<TaskResourceInfo>) -> anyhow::Result<()>;
}

#[derive(Debug)]
pub(crate) enum ResourceManagerWrap {
    LocalResourceManager(LocalResourceManager),
    StandaloneResourceManager(StandaloneResourceManager),
    YarnResourceManager(YarnResourceManager),
}

impl ResourceManagerWrap {
    pub fn new(context: &Context) -> Self {
        match context.cluster_mode {
            ClusterMode::Local => ResourceManagerWrap::LocalResourceManager(
                LocalResourceManager::new(context.clone()),
            ),
            ClusterMode::Standalone => ResourceManagerWrap::StandaloneResourceManager(
                StandaloneResourceManager::new(context.clone()),
            ),
            ClusterMode::YARN => {
                ResourceManagerWrap::YarnResourceManager(YarnResourceManager::new(context.clone()))
            }
        }
    }
}

impl ResourceManager for ResourceManagerWrap {
    fn prepare(&mut self, context: &Context, job_descriptor: &JobDescriptor) {
        match self {
            ResourceManagerWrap::LocalResourceManager(rm) => rm.prepare(context, job_descriptor),
            ResourceManagerWrap::StandaloneResourceManager(rm) => {
                rm.prepare(context, job_descriptor)
            }
            ResourceManagerWrap::YarnResourceManager(rm) => rm.prepare(context, job_descriptor),
        }
    }

    fn worker_allocate<S>(
        &self,
        stream_job: &S,
        stream_env: &StreamExecutionEnvironment,
    ) -> anyhow::Result<Vec<TaskResourceInfo>>
    where
        S: StreamJob + 'static,
    {
        match self {
            ResourceManagerWrap::LocalResourceManager(rm) => {
                rm.worker_allocate(stream_job, stream_env)
            }
            ResourceManagerWrap::StandaloneResourceManager(rm) => {
                rm.worker_allocate(stream_job, stream_env)
            }
            ResourceManagerWrap::YarnResourceManager(rm) => {
                rm.worker_allocate(stream_job, stream_env)
            }
        }
    }

    fn stop_workers(&self, task_ids: Vec<TaskResourceInfo>) -> anyhow::Result<()> {
        match self {
            ResourceManagerWrap::LocalResourceManager(rm) => rm.stop_workers(task_ids),
            ResourceManagerWrap::StandaloneResourceManager(rm) => rm.stop_workers(task_ids),
            ResourceManagerWrap::YarnResourceManager(rm) => rm.stop_workers(task_ids),
        }
    }
}
