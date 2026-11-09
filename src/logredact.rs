use protobuf::PbPrint;
use std::collections::HashMap;

pub(crate) fn cloud_dynamic_attrs(attrs: &HashMap<String, String>, name: &str, buf: &mut String) {
    if attrs.values().all(String::is_empty) {
        PbPrint::fmt(attrs, name, buf);
        return;
    }
    let redacted: HashMap<String, String> = attrs
        .iter()
        .map(|(key, value)| {
            let value = if value.is_empty() {
                String::new()
            } else {
                "[REDACTED]".to_owned()
            };
            (key.clone(), value)
        })
        .collect();
    PbPrint::fmt(&redacted, name, buf);
}
