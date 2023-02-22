use log;
use package_json::PackageJsonManager;

pub type PackageJson = package_json::PackageJson;

pub fn get_local() -> Option<PackageJson> {
    let mut file_path = std::env::current_dir().unwrap();
    file_path.push("package.json");

    let mut manager = PackageJsonManager::with_file_path(file_path);

    match manager.read_ref() {
        Ok(package) => Some(package),
        Err(err) => {
            log::debug!("Failed to read local package.json: {}", err);
            None
        }
    }
}
