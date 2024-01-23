use std::fs;
use std::path::Path;
use std::process::Command;

use crate::*;

pub fn build_bundle()
{
    let bundle_dir_path = Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .parent()
        .unwrap()
        .join("bundle");

    if Path::exists(&bundle_dir_path) {
        fs::remove_dir_all(&bundle_dir_path).unwrap();
    }

    fs::create_dir_all(&bundle_dir_path).unwrap();

    let cargo_bin_file_path = std::env::var("CARGO").unwrap();

    let driver_library_crate_dir_path = Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .parent()
        .unwrap()
        .join("driver-library");

    let driver_dinput8_library_crate_dir_path =
        Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap())
            .parent()
            .unwrap()
            .join("driver-dinput8-library");

    if Command::new(cargo_bin_file_path.clone())
        .current_dir(&driver_library_crate_dir_path)
        .arg("build")
        .arg("--release")
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .code()
        .unwrap()
        != 0
    {
        panic!();
    }

    if Command::new(cargo_bin_file_path.clone())
        .current_dir(&driver_dinput8_library_crate_dir_path)
        .arg("build")
        .arg("--release")
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .code()
        .unwrap()
        != 0
    {
        panic!();
    }

    fs::copy(
        Path::new(&driver_library_crate_dir_path)
            .join("target")
            .join("release")
            .join("lswtss_open_modding_platform_driver_library.dll"),
        Path::new(&bundle_dir_path).join("lswtss-omp-driver.dll"),
    )
    .unwrap();

    fs::copy(
        Path::new(&driver_dinput8_library_crate_dir_path)
            .join("target")
            .join("release")
            .join("lswtss_open_modding_platform_driver_dinput8_library.dll"),
        Path::new(&bundle_dir_path).join("dinput8.dll"),
    )
    .unwrap();

    let mod_info_json_schema_as_json_value = schemars::schema_for!(ModInfo);

    let mod_info_json_schema_as_json =
        serde_json::to_string_pretty(&mod_info_json_schema_as_json_value).unwrap();

    fs::write(
        Path::new(&bundle_dir_path).join("lswtss-omp-mod-info-json-schema.json"),
        mod_info_json_schema_as_json,
    )
    .unwrap();
}
