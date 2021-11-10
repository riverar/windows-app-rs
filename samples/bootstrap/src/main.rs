use ::windows_app::*;

fn main() {
    bootstrap::initialize()
        .and_then(|_| bootstrap::uninitialize())
        .unwrap();
    println!("First windows-app crate app!");
}
