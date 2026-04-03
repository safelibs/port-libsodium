use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const REQUIRED_SYNCED_ARTIFACTS: &[&str] = &[
    "include/sodium.h",
    "cabi/libsodium.map",
    "cabi/expected/foundation.symbols",
    "cabi/expected/through_symmetric.symbols",
    "cabi/expected/through_public_key.symbols",
    "cabi/expected/full.symbols",
    "cabi/expected/upstream-kinds.tsv",
];

const LAYOUT_TYPES: &[(&str, bool)] = &[
    ("crypto_aead_aes256gcm_state", true),
    ("crypto_auth_hmacsha256_state", true),
    ("crypto_auth_hmacsha512_state", true),
    ("crypto_auth_hmacsha512256_state", false),
    ("crypto_generichash_blake2b_state", true),
    ("crypto_generichash_state", false),
    ("crypto_onetimeauth_poly1305_state", true),
    ("crypto_onetimeauth_state", false),
    ("crypto_hash_sha256_state", true),
    ("crypto_hash_sha512_state", true),
    ("crypto_secretstream_xchacha20poly1305_state", true),
    ("crypto_sign_ed25519ph_state", true),
    ("crypto_sign_state", false),
    ("randombytes_implementation", false),
];

const ALIASES: &[(&str, &str)] = &[
    (
        "crypto_auth_hmacsha512256_state",
        "crypto_auth_hmacsha512_state",
    ),
    (
        "crypto_generichash_state",
        "crypto_generichash_blake2b_state",
    ),
    (
        "crypto_onetimeauth_state",
        "crypto_onetimeauth_poly1305_state",
    ),
    ("crypto_sign_state", "crypto_sign_ed25519ph_state"),
];

fn main() {
    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("missing CARGO_MANIFEST_DIR"));

    validate_synced_artifacts(&manifest_dir);

    println!(
        "cargo:rerun-if-changed={}",
        manifest_dir.join("cabi/weak_runtime.c").display()
    );

    let include_dir = manifest_dir.join("include");
    let version_script = manifest_dir.join("cabi/libsodium.map");

    if env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("linux") {
        println!("cargo:rustc-cdylib-link-arg=-Wl,-soname,libsodium.so.23");
        println!("cargo:rustc-cdylib-link-arg=-Wl,--undefined-version");
        println!(
            "cargo:rustc-cdylib-link-arg=-Wl,--version-script={}",
            version_script.display()
        );
    }

    cc::Build::new()
        .file(manifest_dir.join("cabi/weak_runtime.c"))
        .include(&include_dir)
        .warnings(false)
        .compile("sodium_weak_runtime");

    generate_layout_bindings(&manifest_dir, &include_dir);
}

fn validate_synced_artifacts(manifest_dir: &Path) {
    for rel in REQUIRED_SYNCED_ARTIFACTS {
        let path = manifest_dir.join(rel);
        println!("cargo:rerun-if-changed={}", path.display());
        if !path.is_file() {
            panic!(
                "missing synced interface artifact: {}. run safe/tools/sync-upstream-interface.sh first",
                path.display()
            );
        }
    }
}

fn generate_layout_bindings(manifest_dir: &Path, include_dir: &Path) {
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("missing OUT_DIR"));
    let probe_source = out_dir.join("layout_probe.c");
    let probe_binary = out_dir.join("layout_probe");
    let generated = out_dir.join("abi_layout_generated.rs");

    fs::write(&probe_source, layout_probe_source()).expect("failed to write layout probe");

    let compiler = cc::Build::new()
        .include(include_dir)
        .cargo_metadata(false)
        .warnings(false)
        .get_compiler();

    let mut compile = compiler.to_command();
    compile
        .arg("-std=c11")
        .arg("-I")
        .arg(include_dir)
        .arg("-o")
        .arg(&probe_binary)
        .arg(&probe_source);
    run_checked(compile, "compile layout probe");

    let output = Command::new(&probe_binary)
        .output()
        .expect("failed to run layout probe");
    if !output.status.success() {
        panic!(
            "layout probe failed:\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let generated_source = render_layout_module(
        &String::from_utf8(output.stdout).expect("layout probe stdout was not UTF-8"),
    );
    fs::write(&generated, generated_source).expect("failed to write generated ABI layouts");

    println!(
        "cargo:rerun-if-changed={}",
        manifest_dir.join("include/sodium.h").display()
    );
}

fn run_checked(mut command: Command, action: &str) {
    let status = command.status().unwrap_or_else(|err| {
        panic!("failed to {}: {}", action, err);
    });
    if !status.success() {
        panic!("failed to {}: exit status {}", action, status);
    }
}

fn layout_probe_source() -> String {
    let mut src = String::from("#include <stdio.h>\n#include \"sodium.h\"\n\nint main(void)\n{\n");
    for (ty, _) in LAYOUT_TYPES {
        src.push_str(&format!(
            "    printf(\"{ty}\\t%zu\\t%zu\\n\", sizeof({ty}), _Alignof({ty}));\n"
        ));
    }
    src.push_str("    return 0;\n}\n");
    src
}

fn render_layout_module(stdout: &str) -> String {
    let mut rendered = String::new();

    for line in stdout.lines() {
        let mut fields = line.split('\t');
        let ty = fields.next().expect("missing type name");
        let size = fields
            .next()
            .expect("missing type size")
            .parse::<usize>()
            .expect("invalid type size");
        let align = fields
            .next()
            .expect("missing type alignment")
            .parse::<usize>()
            .expect("invalid type alignment");
        let const_prefix = to_upper_snake(ty);

        rendered.push_str(&format!("pub const {const_prefix}_SIZE: usize = {size};\n"));
        rendered.push_str(&format!(
            "pub const {const_prefix}_ALIGN: usize = {align};\n"
        ));
    }

    rendered.push('\n');

    for line in stdout.lines() {
        let mut fields = line.split('\t');
        let ty = fields.next().expect("missing type name");
        let size = fields
            .next()
            .expect("missing type size")
            .parse::<usize>()
            .expect("invalid type size");
        let align = fields
            .next()
            .expect("missing type alignment")
            .parse::<usize>()
            .expect("invalid type alignment");

        if LAYOUT_TYPES
            .iter()
            .find(|(name, _)| *name == ty)
            .map(|(_, generate_struct)| *generate_struct)
            .unwrap_or(false)
        {
            rendered.push_str(&format!(
                "#[repr(C, align({align}))]\npub struct {ty} {{\n    pub(crate) opaque: [u8; {size}],\n}}\n\n"
            ));
        }
    }

    for (alias, target) in ALIASES {
        rendered.push_str(&format!("pub type {alias} = {target};\n"));
    }

    rendered
}

fn to_upper_snake(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut prev_is_lower = false;

    for ch in input.chars() {
        if ch == '_' {
            out.push('_');
            prev_is_lower = false;
            continue;
        }
        if ch.is_ascii_uppercase() && prev_is_lower {
            out.push('_');
        }
        out.push(ch.to_ascii_uppercase());
        prev_is_lower = ch.is_ascii_lowercase() || ch.is_ascii_digit();
    }

    out
}
