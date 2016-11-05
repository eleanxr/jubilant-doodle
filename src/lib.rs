
extern crate rustc_serialize;

use std::collections::HashMap;

use rustc_serialize::json::Json;
use rustc_serialize::json::Object;

enum Metric {
    FloatMetric(f64),
    IntMetric(i64)
}

enum Column {
    DimensionColumn(String),
    MetricColumn(Metric)
}

fn convert_item(name: &String, value: &Json) -> (String, Column) {
    match value {
        &Json::I64(number) => (String::new(name),
                              Column::MetricColumn(
                                  Metric::IntMetric(number))),
        &Json::F64(number) => (String::new(name),
                              Column::MetricColumn(
                                  Metric::FloatMetric(number)))
    }
}

fn map_object(object: &Object) -> HashMap<String, Column> {
    object.iter()
        .map(|(name, value)| convert_item(name, value))
        .collect()
}

fn map_record(record: &Json) -> Option<HashMap<String, Column>> {
    if record.is_object() {
        Some(map_object(record.as_object))
    } else {
        None
    }
}

fn map_records(records: &Vec<Json>) -> HashMap<String, Vec<Column>> {
    let mut columns = HashMap::new();

    for record in records.iter() {
    }

    columns
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
