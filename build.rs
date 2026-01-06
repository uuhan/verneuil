fn main() {
    // from libsqlite3-sys
    let dep_includes = std::env::var("DEP_SQLITE3_INCLUDE").expect("sqlite3 include");

    println!("cargo:rerun-if-changed=c/file_ops.c");
    println!("cargo:rerun-if-changed=c/file_ops.h");
    println!("cargo:rerun-if-changed=c/vfs.c");
    println!("cargo:rerun-if-changed=c/vfs.h");
    println!("cargo:rerun-if-changed=include/verneuil.h");
    let mut build = cc::Build::new();
    build
        .flag_if_supported("-Wmissing-declarations")
        .flag_if_supported("-Wmissing-prototypes")
        .flag_if_supported("-Wstrict-prototypes")
        .flag_if_supported("-Wundef")
        .include(&dep_includes)
        .include("include");

    if cfg!(feature = "test_vfs") {
        // Enable test-only code.
        build
            .define("TEST_VFS", None)
            .define("SQLITE_TEST", None)
            // -fcommon to avoid collisions between test-only counters
            // like `sqlite3_sync_count` that must be defined redundantly
            // in vfs.c (and are always defined in SQLite's test binary).
            //
            // This lets us build libverneuil *once* and use it in SQLite
            // test binaries that do and don't define the counters.
            .flag_if_supported("-fcommon");
    }

    // if cfg!(feature = "dynamic_vfs") {
    // } else {
    //     // We're linking this extension statically, without going
    //     // through sqlite's dynamic loading mechanism.
    //     build.define("SQLITE_CORE", None);
    // }

    // We want GNU extensions on Linux.
    #[cfg(target_os = "linux")]
    build.define("_GNU_SOURCE", None);
    build
        // We know the linuxvfs doesn't implement dirsync.
        .define("SQLITE_DISABLE_DIRSYNC", None)
        .file("c/file_ops.c")
        .file("c/vfs.c")
        .opt_level(2)
        .compile("verneuil_vfs")
}
