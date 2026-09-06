#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    omnitotp_lib::run()
}
