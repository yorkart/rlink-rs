use crate::api::window::WindowWrap;
use crate::runtime::ChainId;
use crate::storage::keyed_state::mem_reducing_state::MemoryReducingState;
use dashmap::DashMap;

lazy_static! {
    static ref DROP_WINDOW_STATE_STORAGE: DashMap<StorageKey, DashMap<WindowWrap, MemoryReducingState>> =
        DashMap::new();
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct StorageKey {
    chain_id: ChainId,
    task_number: u16,
}

impl StorageKey {
    pub fn new(chain_id: ChainId, task_number: u16) -> Self {
        StorageKey {
            chain_id,
            task_number,
        }
    }
}

pub(crate) fn append_drop_window(
    storage_key: StorageKey,
    window: WindowWrap,
    state: MemoryReducingState,
) {
    let drop_window_states: &DashMap<StorageKey, DashMap<WindowWrap, MemoryReducingState>> =
        &*DROP_WINDOW_STATE_STORAGE;

    let task_storage = drop_window_states
        .entry(storage_key)
        .or_insert_with(|| DashMap::new());
    task_storage.value().insert(window, state);
}

pub(crate) fn remove_drop_window(
    chain_id: ChainId,
    task_number: u16,
    window: WindowWrap,
) -> Option<MemoryReducingState> {
    let drop_window_states: &DashMap<StorageKey, DashMap<WindowWrap, MemoryReducingState>> =
        &*DROP_WINDOW_STATE_STORAGE;

    let key = StorageKey::new(chain_id, task_number);
    match drop_window_states.get(&key) {
        Some(task_storage) => task_storage.value().remove(&window).map(|(_k, v)| v),
        None => None,
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::api::element::Record;
//     use crate::api::window::{TimeWindow, WindowWrap};
//     use crate::storage::keyed_state::mem_storage::{drop_window, merge, windows};
//
//     #[test]
//     pub fn parse_test() {
//         let time_window = TimeWindow::new(2, 5);
//         let window_wrap = WindowWrap::TimeWindow(time_window);
//         let time_windows = vec![window_wrap.clone()];
//         let key = Record::new();
//         let val = Record::new();
//         merge(1, 12, &time_windows, key, val, |_x, _b| Record::new());
//
//         for window in windows(1, 12) {
//             println!("1. {:?}", window);
//         }
//
//         drop_window(1, 12, &window_wrap);
//
//         for window in windows(1, 12) {
//             println!("2. {:?}", window);
//         }
//     }
// }
