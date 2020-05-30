// NOTE: below function is build as no_mangle
extern "Rust" {
    fn add(x: i32, y: i32) -> i32;
}

#[no_mangle]
fn add_hook(x: i32, y: i32) -> i32 {
    println!("rust lib hook called!");
    unsafe { add(x, y) + 345 }
}
