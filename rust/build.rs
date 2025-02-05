use std::env;
use std::path::PathBuf;

fn main() {
    let output_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let base_path = env::current_dir().unwrap().join("..").canonicalize().unwrap();
    let include_path = base_path.join("include");
    let include_path = include_path.to_str().unwrap();
    let builder = bindgen::Builder::default()
        .header("src/wrapper.h")
        .clang_args(["-I/usr/arm-none-eabi/include", "-iquote", &format!("{include_path}")])
        .clang_args(["--target=arm-none-eabi", "-mthumb", "-march=armv4t"])
        .allowlist_file(".*/task.h")
        .allowlist_file(".*/malloc.h")
        .allowlist_file(".*/window.h")
        .allowlist_file(".*/main.h")
        .allowlist_file(".*/menu.h")
        .allowlist_file(".*/menu_helpers.h")
        .allowlist_file(".*/bg.h")
        .allowlist_file(".*/sprite.h")
        .allowlist_file(".*/palette.h")
        .allowlist_file(".*/decompress.h")
        .allowlist_file(".*/syscall.h")
        .allowlist_file(".*/isagbprint.h")
        .derive_default(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .use_core();

    let bindings = builder.clone().generate().expect("bindings");
    bindings.write_to_file(output_path.join("bindings.rs")).expect("Writing to file");
}
