
extern crate rustc_serialize;

use std::collections::HashMap;

use rustc_serialize::json::Json;
use rustc_serialize::json::Object;

enum Metric {
    FloatMetric(f64),
    IntMetric(i64),
    UIntMetric(u64)
}

enum Column {
    DimensionColumn(String),
    MetricColumn(Metric)
}

fn convert_item(name: &String, value: &Json) -> Option<(String, Column)> {
    let key = name.clone();
    match value {
        &Json::I64(number) => Some((key,
                              Column::MetricColumn(
                                  Metric::IntMetric(number)))),
        &Json::F64(number) => Some((key,
                              Column::MetricColumn(
                                  Metric::FloatMetric(number)))),
        &Json::U64(number) => Some((key,
                               Column::MetricColumn(
                                   Metric::UIntMetric(number)))),
        &Json::String(ref s) => Some((key,
                             Column::DimensionColumn(s.clone()))),
        _ => None
    }
}

fn map_object(object: &Object) -> HashMap<String, Column> {
    object.iter()
        .map(|(name, value)| convert_item(name, value))
        .filter_map(|x| x)
        .collect()
}

fn map_record(record: &Json) -> Option<HashMap<String, Column>> {
    record.as_object().map(|o| map_object(o))
}

fn map_records(records: &Vec<Json>) -> HashMap<String, Vec<Column>> {
    let mut columns = HashMap::new();

    for record in records.iter() {
        columns.
    }

    columns
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
