use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    if let Ok(_) = std::env::var("DOCS_RS") {
        // Do not try to build anything when generating docs
        return;
    }

    let doslike_path = Path::new("dos-like");

    init_submodule(doslike_path);

    println!("cargo:rerun-if-changed=dos-like/source/dos.c");
    println!("cargo:rerun-if-changed=dos-like/source/dos.h");

    let compiled_lib_path = compile(doslike_path);

    println!("cargo:rustc-link-search={}", compiled_lib_path.display());

    link();
}

fn init_submodule(doslike_path: &Path) {
    if !doslike_path.join("source").exists() {
        Command::new("git")
            .args(&["submodule", "update", "--init"])
            .current_dir(doslike_path)
            .status()
            .expect("Git is needed to retrieve the dos-like source files");
    }
}

fn compile(source_path: &Path) -> PathBuf {
    let include_paths = compute_include_paths("/usr/include/SDL2");

    // statically link dos-like source
    let mut build = cc::Build::new();

    build
        .file(source_path.join("source/dos.c"))
        .warnings(false)
        .extra_warnings(false)
        .includes(include_paths)
        .define("NO_MAIN_DEF", "1");

    if cfg!(feature = "disable-screen-frame") {
        build.define("DISABLE_SCREEN_FRAME", "1");
    }

    if cfg!(feature = "disable-system-cursor") {
        build.define("DISABLE_SYSTEM_CURSOR", "1");        
    }

    if cfg!(target_arch = "wasm32") {
        build.define("__wasm__", "1");
    }

    build.compile("dos-like");

    PathBuf::from("dos-like")
}

fn compute_include_paths(fallback_path: impl AsRef<Path>) -> Vec<PathBuf> {
    let mut include_paths = vec![];

    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let host = std::env::var("HOST").unwrap_or_default();
    let host_os = host.split('-').nth(2).unwrap_or_default();

    if target_arch == "wasm32" && host_os == "linux" {
        include_paths.push(PathBuf::from("/usr/include"));
    }

    if !(target_os == "linux" || target_os == "macos") {
        return include_paths;
    }

    if let Ok(include_path) = env::var("SDL2_INCLUDE_PATH") {
        include_paths.push(PathBuf::from(include_path));
    };

    #[cfg(feature = "pkg-config")]
    {
        // don't print the "cargo:xxx" directives, we're just trying to get the include paths here
        let pkg_config_library = pkg_config::Config::new()
            .print_system_libs(false)
            .probe("sdl2")
            .unwrap();
        for path in pkg_config_library.include_paths {
            include_paths.push(path);
        }
    }

    #[cfg(feature = "vcpkg")]
    {
        // don't print the "cargo:xxx" directives, we're just trying to get the include paths here
        let vcpkg_library = vcpkg::Config::new()
            .cargo_metadata(false)
            .probe("sdl2")
            .unwrap();
        for path in vcpkg_library.include_paths {
            include_paths.push(path);
        }
    }

    if include_paths.is_empty() {
        include_paths.push(fallback_path.as_ref().to_owned());
    }

    include_paths
}

fn link() {
    if !(cfg!(target_os = "linux") || cfg!(target_os = "macos")) {
        return;
    }

    #[cfg(feature = "use-pkgconfig")]
    {
        // prints the appropriate linking parameters when using pkg-config
        get_pkg_config();
    }

    #[cfg(feature = "use-vcpkg")]
    {
        // prints the appropriate linking parameters when using pkg-config
        // useless when using "bundled"
        get_vcpkg_config();
    }

    if cfg!(feature = "use-pkgconfig") == false && cfg!(feature = "use-vcpkg") == false {
        println!("cargo:rustc-flags=-l SDL2main");
        println!("cargo:rustc-flags=-l SDL2");
    }

    println!("cargo:rustc-flags=-l GLEW");
    println!("cargo:rustc-flags=-l GL");
}

#[cfg(feature = "use-pkgconfig")]
fn get_pkg_config() {
    pkg_config_print(true, "sdl2");
}

#[cfg(feature = "use-vcpkg")]
fn get_vcpkg_config() {
    vcpkg::find_package("sdl2").unwrap();
}

#[cfg(feature = "use-pkgconfig")]
fn pkg_config_print(statik: bool, lib_name: &str) {
    pkg_config::Config::new()
        .statik(statik)
        .probe(lib_name)
        .unwrap();
}
