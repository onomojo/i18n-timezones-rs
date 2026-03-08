use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir).join("generated.rs");
    let mut out = fs::File::create(&out_path).expect("failed to create generated.rs");

    let data_dir = Path::new("data");
    let mut entries: Vec<String> = fs::read_dir(data_dir)
        .expect("failed to read data/")
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().into_string().unwrap())
        .filter(|name| name.ends_with(".json"))
        .map(|name| name.trim_end_matches(".json").to_string())
        .collect();
    entries.sort();

    writeln!(out, "pub const ALL_LOCALES: &[(&str, &str)] = &[").unwrap();
    for locale in &entries {
        writeln!(out, "    (\"{locale}\", include_str!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/data/{locale}.json\"))),").unwrap();
    }
    writeln!(out, "];").unwrap();

    println!("cargo:rerun-if-changed=data/");
}
