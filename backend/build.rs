use std::{env, path::PathBuf, process::Command};

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is not set"));
    let frontend_dir = manifest_dir
        .parent()
        .expect("backend directory must have a parent")
        .join("frontend");
    let frontend_dist_dir = frontend_dir.join("dist");
    let package_json = frontend_dir.join("package.json");

    println!("cargo:rerun-if-changed={}", package_json.display());
    println!("cargo:rerun-if-changed={}", frontend_dir.join("src").display());
    println!("cargo:rerun-if-changed={}", frontend_dir.join("public").display());
    println!("cargo:rerun-if-changed={}", frontend_dir.join("index.html").display());
    println!("cargo:rerun-if-changed={}", frontend_dir.join("vite.config.ts").display());
    println!("cargo:rerun-if-changed={}", frontend_dir.join("tsconfig.json").display());
    println!("cargo:rerun-if-env-changed=SKIP_FRONTEND_BUILD");

    if env::var("SKIP_FRONTEND_BUILD").is_ok() {
        if !frontend_dist_dir.exists() {
            panic!(
                "SKIP_FRONTEND_BUILD is set but frontend/dist does not exist. Build the frontend first or unset SKIP_FRONTEND_BUILD."
            );
        }
        return;
    }

    if !package_json.exists() {
        panic!("frontend/package.json not found at {}", package_json.display());
    }

    run_command(&frontend_dir, npm_command(), &["install"]);
    run_command(&frontend_dir, npm_command(), &["run", "build"]);

    if !frontend_dist_dir.exists() {
        panic!(
            "frontend build finished but dist directory was not generated at {}",
            frontend_dist_dir.display()
        );
    }
}

fn run_command(cwd: &PathBuf, command: &str, args: &[&str]) {
    let status = Command::new(command)
        .args(args)
        .current_dir(cwd)
        .status()
        .unwrap_or_else(|error| {
            panic!(
                "failed to execute `{}` in {}: {}",
                format_command(command, args),
                cwd.display(),
                error
            )
        });

    if !status.success() {
        panic!(
            "command `{}` failed in {} with status {}",
            format_command(command, args),
            cwd.display(),
            status
        );
    }
}

fn npm_command() -> &'static str {
    if cfg!(windows) {
        "npm.cmd"
    } else {
        "npm"
    }
}

fn format_command(command: &str, args: &[&str]) -> String {
    if args.is_empty() {
        command.to_string()
    } else {
        format!("{} {}", command, args.join(" "))
    }
}
