use inquire::Select;
use std::collections::HashMap;
use std::fs;
use std::fs::write;

const C: &[u8] = include_bytes!("../resources/c.toml");
const CPP: &[u8] = include_bytes!("../resources/cpp.toml");
const DART_FLUTTER: &[u8] = include_bytes!("../resources/dart_flutter.toml");
const GO: &[u8] = include_bytes!("../resources/go.toml");
const HASKELL: &[u8] = include_bytes!("../resources/haskell.toml");
const JAVA_KOTLIN: &[u8] = include_bytes!("../resources/java_kotlin.toml");
const JS_TS: &[u8] = include_bytes!("../resources/js_ts.toml");
const PHP: &[u8] = include_bytes!("../resources/php.toml");
const PYTHON: &[u8] = include_bytes!("../resources/python.toml");
const RUBY: &[u8] = include_bytes!("../resources/ruby.toml");
const RUST: &[u8] = include_bytes!("../resources/rust.toml");
const SWIFT: &[u8] = include_bytes!("../resources/swift.toml");
const ZIG: &[u8] = include_bytes!("../resources/zig.toml");

pub fn init() {
    if fs::exists("./clean.toml").unwrap() {
        println!("clean.toml file already exists");
        return;
    }

    let mut map = HashMap::new();
    map.insert("C", C);
    map.insert("C++", CPP);
    map.insert("Dart / Flutter", DART_FLUTTER);
    map.insert("GO", GO);
    map.insert("Haskell", HASKELL);
    map.insert("Java / Kotlin", JAVA_KOTLIN);
    map.insert("Javascript / Typescript", JS_TS);
    map.insert("PHP", PHP);
    map.insert("Python", PYTHON);
    map.insert("Ruby", RUBY);
    map.insert("Rust", RUST);
    map.insert("Swift", SWIFT);
    map.insert("Zig", ZIG);

    let mut opts = map.keys().collect::<Vec<_>>();
    opts.sort();

    let res = Select::new("What type of project do you want to clean ?", opts)
        .prompt()
        .unwrap();

    write("./clean.toml", map.get(res).unwrap()).unwrap();
}
