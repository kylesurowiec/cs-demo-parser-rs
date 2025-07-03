use prost::Message;
use std::error::Error;

use crate::proto::msg::{CDataGccStrike15V2MatchInfo, WatchableMatchInfo};

/// Extracts the net-message decryption key stored in `match730_*.dem.info` files.
/// Pass the whole contents of the `.dem.info` file to this function to get the key.
pub fn match_info_decryption_key(bytes: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let info = CDataGccStrike15V2MatchInfo::decode(bytes)?;
    let key = info
        .watchablematchinfo
        .and_then(|w| w.cl_decryptdata_key_pub)
        .ok_or("missing decrypt key")?;
    Ok(format!("{:016X}", key).into_bytes())
}
