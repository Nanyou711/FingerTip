fn main() {
    tauri_build::build();
    println!("cargo:rerun-if-env-changed=FINGERTIP_MINIMAX_KEY");
}
