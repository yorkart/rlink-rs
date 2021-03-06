use crate::api::element::Record;
use crate::api::function::{Context, Function, KeySelectorFunction};
use crate::functions::column_base_function::FunctionSchema;

#[derive(Debug)]
pub struct ColumnBaseKeySelector {
    data_types: Vec<u8>,
    key_data_types: Vec<u8>,
    columns: Vec<usize>,
}

impl ColumnBaseKeySelector {
    pub fn new(columns: Vec<usize>, data_types: Vec<u8>) -> Self {
        let key_data_types: Vec<u8> = columns.iter().map(|index| data_types[*index]).collect();
        ColumnBaseKeySelector {
            columns,
            data_types,
            key_data_types,
        }
    }
}

impl FunctionSchema for ColumnBaseKeySelector {
    fn get_schema_types(&self) -> Vec<u8> {
        self.key_data_types.clone()
    }
}

impl KeySelectorFunction for ColumnBaseKeySelector {
    fn open(&mut self, _context: &Context) {}

    fn get_key(&self, record: &mut Record) -> Record {
        let mut record_key = Record::with_capacity(record.len());
        let mut writer = record_key.get_writer(self.key_data_types.as_slice());

        let mut reader = record.get_reader(self.data_types.as_slice());

        for index in 0..self.columns.len() {
            writer
                .set_bytes_raw(reader.get_bytes_raw(self.columns[index]).unwrap())
                .unwrap();
        }

        record_key
    }

    fn close(&mut self) {}
}

impl Function for ColumnBaseKeySelector {
    fn get_name(&self) -> &str {
        "ColumnBaseKeySelector"
    }
}
