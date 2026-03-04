use rhai::{Engine, Scope, Dynamic};

fn main() -> Result<(), Box<rhai::EvalAltResult>> {
    let mut engine = Engine::new();

    // 脚本里定义函数
    let ast = engine.compile(r#"
        fn add(a, b) {
            a + b
        }

        fn hello(name) {
            "hello " + name
        }
    "#)?;

    // 通过函数名调用（字符串）
    let result: i64 = engine.call_fn(
        &mut Scope::new(),
        &ast,
        "add",
        (40_i64, 2_i64),
    )?;

    let result2: String = engine.call_fn(
        &mut Scope::new(),
        &ast,
        "hello",
        ("rust",),
    )?;

    println!("{result}, {result2}");
    Ok(())
}


# 如何让 Rhai「读文件」
Rust 端注册函数

use std::fs;
use rhai::Engine;

fn read_file(path: &str) -> Result<String, Box<rhai::EvalAltResult>> {
    fs::read_to_string(path)
        .map_err(|e| format!("read file error: {e}").into())
}

fn main() {
    let mut engine = Engine::new();
    engine.register_fn("read_file", read_file);

    let result: String = engine
        .eval(r#" read_file("config.txt") "#)
        .unwrap();

    println!("{result}");
}



Rhai 脚本侧
let content = read_file("config.txt");