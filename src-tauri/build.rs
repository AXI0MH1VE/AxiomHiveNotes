fn main() {
  // Enable FTS5 and JSON1 extensions for SQLite
  println!("cargo:rustc-env=LIBSQLITE3_FLAGS=-DSQLITE_ENABLE_FTS5 -DSQLITE_ENABLE_JSON1");
  
  tauri_build::build()
}
