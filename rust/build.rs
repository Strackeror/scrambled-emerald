use std::env;

fn main() {
    let base_path = env::current_dir()
        .unwrap()
        .join("..")
        .canonicalize()
        .unwrap();
    let include_path = base_path.join("include");
    let include_path = include_path.to_str().unwrap();
    let bindings = bindgen::Builder::default()
        .header("src/wrapper.h")
        .clang_args(["-iquote", &format!("{include_path}")])
        .clang_arg("--target=thumbv4t-none-eabi")
        .clang_arg("-I/usr/arm-none-eabi/include")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .use_core()
        .opaque_type("PokemonSubstruct3")
        .opaque_type("Berry")
        .opaque_type("Berry2")
        .opaque_type("ObjectEventTemplate")
        .opaque_type("SaveBlock1")
        .generate()
        .expect("bindings");
    bindings
        .write_to_file("src/bindings.rs")
        .expect("Writing to file");
}
