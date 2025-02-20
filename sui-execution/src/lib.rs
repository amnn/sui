// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use sui_protocol_config::ProtocolConfig;
use sui_types::{error::SuiResult, metrics::BytecodeVerifierMetrics};

pub use executor::Executor;
pub use verifier::Verifier;

pub mod executor;
pub mod verifier;

mod latest;
mod v0;

pub const LATEST_EXECUTOR_VERSION: u64 = 1;

pub fn executor(
    protocol_config: &ProtocolConfig,
    paranoid_type_checks: bool,
    silent: bool,
) -> SuiResult<Arc<dyn Executor + Send + Sync>> {
    let version = protocol_config.execution_version_as_option().unwrap_or(0);
    Ok(match version {
        0 => Arc::new(v0::Executor::new(
            protocol_config,
            paranoid_type_checks,
            silent,
        )?),

        1 => Arc::new(latest::Executor::new(
            protocol_config,
            paranoid_type_checks,
            silent,
        )?),

        v => panic!("Unsupported execution version {v}"),
    })
}

pub fn verifier<'m>(
    protocol_config: &ProtocolConfig,
    is_metered: bool,
    metrics: &'m Arc<BytecodeVerifierMetrics>,
) -> Box<dyn Verifier + 'm> {
    let version = protocol_config.execution_version_as_option().unwrap_or(0);
    match version {
        0 => Box::new(v0::Verifier::new(protocol_config, is_metered, metrics)),
        1 => Box::new(latest::Verifier::new(protocol_config, is_metered, metrics)),
        v => panic!("Unsupported execution version {v}"),
    }
}
