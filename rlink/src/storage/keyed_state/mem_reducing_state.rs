use crate::api::element::Record;
use crate::storage::keyed_state::mem_storage::remove_drop_window;
use crate::storage::keyed_state::{ReducingState, StateIterator, StateKey};
use std::collections::HashMap;

// type RecordBuildHasher = std::hash::BuildHasherDefault<RecordHasher>;

#[derive(Clone, Debug)]
pub struct MemoryReducingState {
    kv: HashMap<Record, Record>,
}

impl MemoryReducingState {
    pub fn new(state_key: &StateKey, suggest_capacity: usize) -> Self {
        debug!(
            "create memory state {:?}, suggest capacity {}",
            state_key, suggest_capacity
        );
        MemoryReducingState {
            kv: HashMap::with_capacity(suggest_capacity),
            // kv: HashMap::<Record, Record, RecordBuildHasher>::default(),
        }
    }

    pub fn from(state_key: &StateKey) -> Option<MemoryReducingState> {
        let state = remove_drop_window(
            state_key.chain_id,
            state_key.task_number,
            state_key.window.clone(),
        );
        if state.is_some() {
            debug!("remove state {:?}", state_key);
        } else {
            error!("can not found state {:?}", state_key);
        }

        state
    }
}

impl ReducingState for MemoryReducingState {
    fn get_mut(&mut self, key: &Record) -> Option<&mut Record> {
        self.kv.get_mut(key)
    }

    fn insert(&mut self, key: Record, val: Record) {
        self.kv.insert(key, val);
    }

    fn flush(&mut self) {}

    fn snapshot(&mut self) {}

    fn close(self) {}

    fn destroy(self) {}

    fn iter(&self) -> StateIterator {
        StateIterator::HashMap(self.kv.iter())
    }

    fn len(&self) -> usize {
        self.kv.len()
    }
}
