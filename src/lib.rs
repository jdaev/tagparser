mod utils;

use avro_rs::types;

use avro_rs::Schema;
use avro_rs::Writer;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    utils::set_panic_hook();
    let tag_schema = r#"
    {
        "type": "record",
        "name": "tag",
        "fields": [
            {"name": "name", "type": "string"},
            {"name": "value", "type": "string"}
        ]
    }
    "#;

    let tag_list_schema = r#"
    {
        "type": "array",
        "items": "tag"
    }
    "#;
    //alert(&format!("Tag Schema, {}!", tag_schema));
    // if the schemas are not valid, this function will return an error
    let tag = Schema::parse_str(tag_schema).unwrap();
    //let tagList = Schema::parse_str(tag_list_schema).unwrap();

    // schemas can be printed for debugging

    // a writer needs a schema and something to write to
    let mut writer = Writer::new(&tag, Vec::new());
    alert(&format!("Tag, {:#?}!", tag));

    // the Record type models our Record schema
    let mut record = types::Record::new(writer.schema()).unwrap();
    record.put("name", "App-Name");
    record.put("value", "Ardrive-Web");

    // schema validation happens here
    writer.append(record).unwrap();

    // this is how to get back the resulting avro bytecode
    // this performs a flush operation to make sure data has been written, so it can fail
    // you can also call `writer.flush()` yourself without consuming the writer
    let encoded = writer.into_inner().unwrap();
    alert(&format!("Binary, {:#?}!", encoded));
}
