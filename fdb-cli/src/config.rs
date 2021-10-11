use crate::result::Result;
use crate::error::Error;
use os_type::OSType;
use std::fmt::Formatter;
use serde::{Deserialize, Serialize};
use tracing::*;

const CONFIGURATION_NAME: &str = "Config";

#[derive(Serialize, Deserialize, Debug)]
pub struct FdbCliConfig {
    // fdb-cli version
    version: u8,

    // path to cluster file
    pub cluster_file: String,
}

pub fn load_config() -> Result<FdbCliConfig> {
    let config = match confy::load::<FdbCliConfig>(&CONFIGURATION_NAME) {
        Ok(res) => {
            info!("Found fdb-cli configuration file (version: {:?})", res.version);
            res
        },
        Err(e) => return Err(
            Error::UnableToReadConfig(e)
        )
    };
    Ok(config)
}

impl std::fmt::Display for FdbCliConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Configuration values: \nversion: {:?} \ncluster file: {}", self.version, &self.cluster_file)
    }
}

impl std::default::Default for FdbCliConfig {
    fn default() -> Self {
        let path = FdbCliConfig::default_cluster_file();

        Self {
            version: 0,
            cluster_file: String::from(path)
        }
    }
}

impl FdbCliConfig {
    pub fn default_cluster_file() -> &'static str {
        let os_type = os_type::current_platform().os_type;
        match os_type {
            // OS Path
            os_type::OSType::OSX => {
                "/usr/local/etc/foundationdb/fdb.cluster"
            }
            // All other types are unix based systems
            _ => {
                "/etc/foundationdb/fdb.cluster"
            }
        }
    }

    pub fn write(&self) -> Result<()> {
        match confy::store(&CONFIGURATION_NAME, self) {
            Ok(()) => Ok(()),
            Err(e) => return Err(
                Error::UnableToWriteConfig(e)
            )
        }
    }
}