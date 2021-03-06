use std::collections::HashMap;

use rdkafka::Offset;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PartitionMetadata {
    pub(crate) topic: String,
    pub(crate) partition: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OffsetMetadata {
    pub(crate) topic: String,
    pub(crate) partition: i32,
    pub(crate) offset: i64,
}

#[derive(Debug)]
pub struct KafkaSourceStateCache {
    partition_offsets: HashMap<PartitionMetadata, OffsetMetadata>,
}

impl KafkaSourceStateCache {
    pub fn new() -> Self {
        KafkaSourceStateCache {
            partition_offsets: HashMap::new(),
        }
    }

    pub fn update(&mut self, topic: String, partition: i32, offset: i64) {
        let key = PartitionMetadata {
            topic: topic.clone(),
            partition,
        };
        let val = OffsetMetadata {
            topic,
            partition,
            offset: Offset::Offset(offset).to_raw(),
        };

        self.partition_offsets.insert(key, val);
    }

    pub fn snapshot(&self) -> HashMap<PartitionMetadata, OffsetMetadata> {
        self.partition_offsets.clone()
    }

    pub fn get(&self, topic: String, partition: i32, default_offset: Offset) -> OffsetMetadata {
        let key = PartitionMetadata {
            topic: topic.clone(),
            partition,
        };
        match self.partition_offsets.get(&key) {
            Some(offset_metadata) => offset_metadata.clone(),
            None => OffsetMetadata {
                topic,
                partition,
                offset: default_offset.to_raw(),
            },
        }
    }
}
