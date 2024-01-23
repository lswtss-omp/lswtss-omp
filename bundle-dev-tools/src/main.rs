mod build_bundle;
mod debug_bundle;

use std::path::Path;

pub use build_bundle::*;
pub use debug_bundle::*;
pub use lswtss_open_modding_platform_foundation::*;

fn main()
{
    let dev_tools_args = clap::command!()
        .subcommand(clap::Command::new("build-bundle"))
        .subcommand(
            clap::Command::new("debug-bundle").arg(
                clap::Arg::new("lswtssDirPath")
                    .long("lswtssDirPath")
                    .short('l')
                    .value_name("VALUE")
                    .required(true),
            ),
        )
        .get_matches();

    if let Some(_build_bundle_command_args) = dev_tools_args.subcommand_matches("build-bundle") {
        build_bundle();
    } else if let Some(debug_bundle_command_args) =
        dev_tools_args.subcommand_matches("debug-bundle")
    {
        let lswtss_dir_path_as_string = debug_bundle_command_args
            .get_one::<String>("lswtssDirPath")
            .unwrap();

        let lswtss_dir_path = Path::new(lswtss_dir_path_as_string);

        debug_bundle(lswtss_dir_path);
    }
}
