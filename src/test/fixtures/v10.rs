use crate::test::fixtures::*;
use crate::{Event, EventBuilder, EventBuilderV10};
use base64::prelude::*;
use serde_json::{json, Value};
use url::Url;

pub fn minimal() -> Event {
    EventBuilderV10::new()
        .id(id())
        .source(source())
        .ty(ty())
        .build()
        .unwrap()
}

pub fn minimal_string_extension() -> Event {
    EventBuilderV10::new()
        .id(id())
        .source(source())
        .ty(ty())
        .extension("someint", "10")
        .build()
        .unwrap()
}

pub fn minimal_json() -> Value {
    json!({
        "specversion": "1.0",
        "id": id(),
        "type": ty(),
        "source": source(),
    })
}

pub fn full_no_data() -> Event {
    let (string_ext_name, string_ext_value) = string_extension();
    let (bool_ext_name, bool_ext_value) = bool_extension();
    let (int_ext_name, int_ext_value) = int_extension();

    EventBuilderV10::new()
        .id(id())
        .source(source())
        .ty(ty())
        .subject(subject())
        .time(time())
        .extension(&string_ext_name, string_ext_value)
        .extension(&bool_ext_name, bool_ext_value)
        .extension(&int_ext_name, int_ext_value)
        .build()
        .unwrap()
}

pub fn full_no_data_json() -> Value {
    let (string_ext_name, string_ext_value) = string_extension();
    let (bool_ext_name, bool_ext_value) = bool_extension();
    let (int_ext_name, int_ext_value) = int_extension();

    json!({
        "specversion": "1.0",
        "id": id(),
        "type": ty(),
        "source": source(),
        "subject": subject(),
        "time": time(),
        string_ext_name: string_ext_value,
        bool_ext_name: bool_ext_value,
        int_ext_name: int_ext_value
    })
}

pub fn full_json_data() -> Event {
    let (string_ext_name, string_ext_value) = string_extension();
    let (bool_ext_name, bool_ext_value) = bool_extension();
    let (int_ext_name, int_ext_value) = int_extension();

    EventBuilderV10::new()
        .id(id())
        .source(source())
        .ty(ty())
        .subject(subject())
        .time(time())
        .extension(&string_ext_name, string_ext_value)
        .extension(&bool_ext_name, bool_ext_value)
        .extension(&int_ext_name, int_ext_value)
        .data_with_schema(
            json_datacontenttype(),
            Url::parse(dataschema().as_ref()).unwrap(),
            json_data(),
        )
        .build()
        .unwrap()
}

pub fn full_json_data_string_extension() -> Event {
    let (string_ext_name, string_ext_value) = string_extension();
    let (bool_ext_name, bool_ext_value) = bool_extension();
    let (int_ext_name, int_ext_value) = int_extension();

    EventBuilderV10::new()
        .id(id())
        .source(source())
        .ty(ty())
        .subject(subject())
        .time(time())
        .extension(&string_ext_name, string_ext_value)
        .extension(&bool_ext_name, bool_ext_value.to_string())
        .extension(&int_ext_name, int_ext_value.to_string())
        .data(json_datacontenttype(), json_data())
        .build()
        .unwrap()
}

pub fn full_binary_json_data_string_extension() -> Event {
    let (string_ext_name, string_ext_value) = string_extension();
    let (bool_ext_name, bool_ext_value) = bool_extension();
    let (int_ext_name, int_ext_value) = int_extension();

    EventBuilderV10::new()
        .id(id())
        .source(source())
        .ty(ty())
        .subject(subject())
        .time(time())
        .extension(&string_ext_name, string_ext_value)
        .extension(&bool_ext_name, bool_ext_value.to_string())
        .extension(&int_ext_name, int_ext_value.to_string())
        .data(json_datacontenttype(), json_data().to_string().into_bytes())
        .build()
        .unwrap()
}

pub fn full_json_data_json() -> Value {
    let (string_ext_name, string_ext_value) = string_extension();
    let (bool_ext_name, bool_ext_value) = bool_extension();
    let (int_ext_name, int_ext_value) = int_extension();

    json!({
        "specversion": "1.0",
        "id": id(),
        "type": ty(),
        "source": source(),
        "subject": subject(),
        "time": time(),
        string_ext_name: string_ext_value,
        bool_ext_name: bool_ext_value,
        int_ext_name: int_ext_value,
        "datacontenttype": json_datacontenttype(),
        "dataschema": dataschema(),
        "data": json_data()
    })
}

pub fn full_json_base64_data_json() -> Value {
    let (string_ext_name, string_ext_value) = string_extension();
    let (bool_ext_name, bool_ext_value) = bool_extension();
    let (int_ext_name, int_ext_value) = int_extension();

    json!({
        "specversion": "1.0",
        "id": id(),
        "type": ty(),
        "source": source(),
        "subject": subject(),
        "time": time(),
        string_ext_name: string_ext_value,
        bool_ext_name: bool_ext_value,
        int_ext_name: int_ext_value,
        "datacontenttype": json_datacontenttype(),
        "dataschema": dataschema(),
        "data_base64": BASE64_STANDARD.encode(json_data_binary())
    })
}

pub fn full_non_json_base64_data() -> Value {
    match full_json_base64_data_json() {
        Value::Object(mut m) => {
            m.insert(
                "data_base64".to_string(),
                Value::String(BASE64_STANDARD.encode(b"hello world")),
            );
            Value::Object(m)
        }
        _ => Value::Null,
    }
}

pub fn full_non_json_data() -> Event {
    let mut event = full_json_data();
    let value = full_non_json_base64_data();
    if let Value::Object(m) = value {
        event.set_data_unchecked(
            BASE64_STANDARD
                .decode(m["data_base64"].as_str().unwrap())
                .unwrap(),
        );
    }
    event
}

pub fn full_xml_string_data() -> Event {
    let (string_ext_name, string_ext_value) = string_extension();
    let (bool_ext_name, bool_ext_value) = bool_extension();
    let (int_ext_name, int_ext_value) = int_extension();

    EventBuilderV10::new()
        .id(id())
        .source(source())
        .ty(ty())
        .subject(subject())
        .time(time())
        .extension(&string_ext_name, string_ext_value)
        .extension(&bool_ext_name, bool_ext_value)
        .extension(&int_ext_name, int_ext_value)
        .data(xml_datacontenttype(), xml_data())
        .build()
        .unwrap()
}

pub fn full_xml_binary_data() -> Event {
    let (string_ext_name, string_ext_value) = string_extension();
    let (bool_ext_name, bool_ext_value) = bool_extension();
    let (int_ext_name, int_ext_value) = int_extension();

    EventBuilderV10::new()
        .id(id())
        .source(source())
        .ty(ty())
        .subject(subject())
        .time(time())
        .extension(&string_ext_name, string_ext_value)
        .extension(&bool_ext_name, bool_ext_value)
        .extension(&int_ext_name, int_ext_value)
        .data(xml_datacontenttype(), Vec::from(xml_data()))
        .build()
        .unwrap()
}

pub fn full_xml_string_data_json() -> Value {
    let (string_ext_name, string_ext_value) = string_extension();
    let (bool_ext_name, bool_ext_value) = bool_extension();
    let (int_ext_name, int_ext_value) = int_extension();

    json!({
        "specversion": "1.0",
        "id": id(),
        "type": ty(),
        "source": source(),
        "subject": subject(),
        "time": time(),
        string_ext_name: string_ext_value,
        bool_ext_name: bool_ext_value,
        int_ext_name: int_ext_value,
        "datacontenttype": xml_datacontenttype(),
        "data": xml_data()
    })
}

pub fn full_xml_base64_data_json() -> Value {
    let (string_ext_name, string_ext_value) = string_extension();
    let (bool_ext_name, bool_ext_value) = bool_extension();
    let (int_ext_name, int_ext_value) = int_extension();

    json!({
        "specversion": "1.0",
        "id": id(),
        "type": ty(),
        "source": source(),
        "subject": subject(),
        "time": time(),
        string_ext_name: string_ext_value,
        bool_ext_name: bool_ext_value,
        int_ext_name: int_ext_value,
        "datacontenttype": xml_datacontenttype(),
        "data_base64": BASE64_STANDARD.encode(Vec::from(xml_data()))
    })
}
