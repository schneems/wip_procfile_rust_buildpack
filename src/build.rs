use libcnb::data::launch::{Launch, Process};
use libcnb::{BuildContext, GenericPlatform};
use std::{fs};
use libcnb::{Error, TomlFileError};

// use crate::error::JvmFunctionInvokerBuildpackError;
use crate::JvmFunctionInvokerBuildpackMetadata;

#[derive(thiserror::Error, Debug)]
pub enum JvmFunctionInvokerBuildpackError {
    #[error("Could not write launch.toml: {0}")]
    CouldNotWriteLaunchToml(TomlFileError),
}

pub fn build(
    context: BuildContext<GenericPlatform, JvmFunctionInvokerBuildpackMetadata>,
) -> Result<(), libcnb::Error<JvmFunctionInvokerBuildpackError>> {
    // let source = context.buildpack_dir.join("opt").join("run.sh");
    let procfile_path = context.app_dir.join("Procfile");
    let contents = fs::read_to_string(procfile_path);
    // println!("{}", procfile_path);
    // println!("{}", contents.unwrap());

    let launch = Launch::default().process(Process::new(
        "web",
        contents.unwrap(),
        vec![
            String::from("")
        ],
        false,
    )?);

    context
        .write_launch(launch)
        .map_err(JvmFunctionInvokerBuildpackError::CouldNotWriteLaunchToml)?;

    Ok(())
}
