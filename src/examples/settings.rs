use serde::{Deserialize, Serialize};

use krossbar_settings_lib::Settings;

#[derive(Serialize, Deserialize)]
struct ComplexStructure {
    integer: i32,
    string: String,
    vec: Vec<bool>,
}

fn main() {
    let storage = Settings::init("krossbar.viewer.example").unwrap();

    storage.read_or_insert("int", 42).unwrap();

    storage
        .read_or_insert("string", "Hello, world!".to_owned())
        .unwrap();

    storage
        .read_or_insert(
            "struct",
            ComplexStructure {
                integer: 42,
                string: "Test".into(),
                vec: vec![true, false],
            },
        )
        .unwrap();
}
