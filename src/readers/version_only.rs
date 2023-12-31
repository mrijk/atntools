use serde::{Deserialize, Serialize};

use super::serializable::MySerializable;


#[derive(Serialize, Deserialize)]
struct VersionOnly{
    version: u32
}

pub fn read_version_only_action_file(version: u32) -> Box<dyn MySerializable>  {
    Box::new(VersionOnly{
        version
    })
}