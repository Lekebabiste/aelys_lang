use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=../Cargo.toml");

    let version = env!("CARGO_PKG_VERSION");
    let workspace_root = Path::new("..").canonicalize().unwrap();

    let updates = vec![
        ("README.md", r"# aelys \d+\.\d+\.\d+-[a-z]", format!("# aelys {}", version)),
        ("examples/hello.aelys", r"Aelys v\d+\.\d+\.\d+-[a-z]", format!("Aelys v{}", version)),
        ("docs/installation.md", r"\d+\.\d+\.\d+-[a-z]", version.to_string()),
    ];

    for (file_path, pattern, replacement) in updates {
        let full_path = workspace_root.join(file_path);
        if full_path.exists() {
            let content = fs::read_to_string(&full_path).unwrap();
            let re = regex::Regex::new(pattern).unwrap();
            let new_content = re.replace_all(&content, replacement.as_str());

            if content != new_content {
                fs::write(&full_path, new_content.as_ref()).unwrap();
                println!("Updated version in {}", file_path);
            }
        }
    }

    update_workspace_crate_versions(version, &workspace_root);
}

fn update_workspace_crate_versions(version: &str, workspace_root: &Path) {
    let crates = [
        "aelys", "aelys-common", "aelys-syntax", "aelys-frontend",
        "aelys-sema", "aelys-opt", "aelys-bytecode", "aelys-backend",
        "aelys-runtime", "aelys-modules", "aelys-driver", "aelys-cli",
        "aelys-native", "aelys-native-macros",
    ];

    for crate_name in crates {
        let cargo_path = workspace_root.join(format!("{}/Cargo.toml", crate_name));
        if cargo_path.exists() {
            let content = fs::read_to_string(&cargo_path).unwrap();
            let mut doc = content.parse::<toml_edit::DocumentMut>().unwrap();

            if let Some(package) = doc.get_mut("package") {
                if let Some(table) = package.as_table_mut() {
                    table["version"] = toml_edit::value(version);
                }
            }

            fs::write(&cargo_path, doc.to_string()).unwrap();
            println!("Updated version in {}/Cargo.toml", crate_name);
        }
    }
}
