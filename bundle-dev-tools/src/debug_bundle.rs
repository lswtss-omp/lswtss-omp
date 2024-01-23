use std::path::Path;
use std::process::Command;

use crate::*;

pub fn debug_bundle(lswtss_dir_path: &Path)
{
    build_bundle();

    let bundle_dir_path = Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .parent()
        .unwrap()
        .join("bundle");

    for bundle_entry_path in std::fs::read_dir(bundle_dir_path.clone()).unwrap() {
        let bundle_entry_path = bundle_entry_path.unwrap();

        std::fs::copy(
            bundle_entry_path.path(),
            lswtss_dir_path.join(
                bundle_entry_path
                    .path()
                    .file_name()
                    .unwrap(),
            ),
        )
        .unwrap();
    }

    let lswtss_executable_file_name = "LEGOSTARWARSSKYWALKERSAGA_DX11.exe".to_string();

    let lswtss_executable_file_path = lswtss_dir_path.join(&lswtss_executable_file_name);

    Command::new(lswtss_executable_file_path)
        .current_dir(lswtss_dir_path)
        .spawn()
        .unwrap();

    let cargo_bin_file_path = std::env::var("CARGO").unwrap();

    let driver_debug_console_crate_dir_path =
        Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap())
            .parent()
            .unwrap()
            .join("driver-debug-console");

    let mut driver_debug_console_process_handle = Command::new(cargo_bin_file_path.clone())
        .current_dir(&driver_debug_console_crate_dir_path)
        .arg("run")
        .spawn()
        .unwrap();

    // When launching Steam version of the game it might be restarted so we need to wait for second process instance.
    std::thread::sleep(std::time::Duration::from_secs(10));

    loop {
        let sysinfo_system_handle = sysinfo::System::new_all();

        let does_lswtss_process_exist = sysinfo_system_handle
            .processes_by_exact_name(&lswtss_executable_file_name)
            .next()
            .is_some();

        if !does_lswtss_process_exist {
            break;
        }

        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    driver_debug_console_process_handle
        .kill()
        .unwrap();
}
