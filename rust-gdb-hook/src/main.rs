#![crate_type = "dylib"]
#[no_mangle]
fn add(x: i32, y: i32) -> i32 {
    println!("original add!");
    x + y
}

#[no_mangle]
fn sub(x: i32, y: i32) -> i32 {
    println!("original sub!");
    x - y
}

#[allow(dead_code)]
#[no_mangle]
fn sub_hook(x: i32, y: i32) -> i32 {
    println!("inner sub hook called!");
    sub(x, y) + 256
}

fn main() {
    let x = 1;
    let y = 2;
    let z = add(x, y);
    let w = sub(x, y);
    println!("z={}", z);
    println!("w={}", w);
}
