use libcnb::data::launch::{Launch, Process};
use libcnb::{BuildContext, GenericPlatform};
use std::fs;

use crate::error::JvmFunctionInvokerBuildpackError;
use crate::JvmFunctionInvokerBuildpackMetadata;

pub fn build(
    context: BuildContext<GenericPlatform, JvmFunctionInvokerBuildpackMetadata>,
) -> Result<(), libcnb::Error<JvmFunctionInvokerBuildpackError>> {
    // let source = context.buildpack_dir.join("opt").join("run.sh");
    let procfile_path = context.app_dir.join("Procfile");
    let contents = fs::read_to_string(procfile_path).unwrap();


    fs::create_dir_all(context.layers_dir.join("procfile"));
    fs::write(
        context.layers_dir.join("launch.toml"),
        "
[[processes]]
type = \"web\"
command = \"while true; do echo 'lol'; sleep 2; done\"
");
    println!("{}", contents.as_str());

    // let launch = Launch::default().process(Process::new(
    //     "web",
    //     contents,
    //     vec![String::from("")],
    //     false,
    // )?);
    //
    // context
    //     .write_launch(launch)
    //     .map_err(JvmFunctionInvokerBuildpackError::CouldNotWriteLaunchToml)?;

    Ok(())
}
