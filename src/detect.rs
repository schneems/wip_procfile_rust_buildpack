use libcnb::data::build_plan::BuildPlanBuilder;
use libcnb::{DetectContext, DetectOutcome, Error, GenericPlatform};

use crate::error::JvmFunctionInvokerBuildpackError;
use crate::JvmFunctionInvokerBuildpackMetadata;

pub fn detect(
    context: DetectContext<GenericPlatform, JvmFunctionInvokerBuildpackMetadata>,
) -> Result<DetectOutcome, Error<JvmFunctionInvokerBuildpackError>> {
    if context.app_dir.join("Procfile").exists() {
        Ok(DetectOutcome::Pass(
            BuildPlanBuilder::new()
                .build(),
        ))
    } else {
        Ok(DetectOutcome::Fail)
    }
}
