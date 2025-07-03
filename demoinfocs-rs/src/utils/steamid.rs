use std::num::ParseIntError;

/// Error type returned by SteamID conversion functions.
#[derive(Debug, PartialEq, Eq)]
pub enum SteamIdError {
    /// The given string does not follow the expected `STEAM_X:Y:Z` format.
    InvalidFormat,
    /// An integer component of the SteamID could not be parsed.
    ParseError(ParseIntError),
}

/// Converts a textual SteamID (e.g. `STEAM_0:1:26343269`) into a 32-bit variant.
///
/// Returns [`SteamIdError`] if the input is not well formed or cannot be parsed.
pub fn convert_steam_id_txt_to_32(steam_id: &str) -> Result<u32, SteamIdError> {
    let trimmed = steam_id.trim_end_matches(']');
    let parts: Vec<&str> = trimmed.split(':').collect();
    if parts.len() != 3 {
        return Err(SteamIdError::InvalidFormat);
    }

    let y = parts[1].parse::<u32>().map_err(SteamIdError::ParseError)?;
    let z = parts[2].parse::<u32>().map_err(SteamIdError::ParseError)?;

    Ok((z << 1) + y)
}

const STEAM_ID64_INDIVIDUAL_IDENTIFIER: u64 = 0x0110_0001_0000_0000;

/// Converts a 32-bit SteamID to the 64-bit variant.
pub fn convert_steam_id32_to_64(steam_id32: u32) -> u64 {
    STEAM_ID64_INDIVIDUAL_IDENTIFIER + steam_id32 as u64
}

/// Converts a 64-bit SteamID to the 32-bit variant.
pub fn convert_steam_id64_to_32(steam_id64: u64) -> u32 {
    (steam_id64 - STEAM_ID64_INDIVIDUAL_IDENTIFIER) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_txt_to_32() {
        let id = convert_steam_id_txt_to_32("STEAM_0:1:26343269").unwrap();
        assert_eq!(52686539, id);
    }

    #[test]
    fn test_convert_txt_to_32_error() {
        assert!(convert_steam_id_txt_to_32("STEAM_0:1:a").is_err());
        assert!(convert_steam_id_txt_to_32("STEAM_0:b:21643603").is_err());
        assert!(convert_steam_id_txt_to_32("STEAM_0:b").is_err());
    }

    #[test]
    fn test_convert_32_to_64() {
        let id = convert_steam_id32_to_64(52686539);
        assert_eq!(76561198012952267u64, id);
    }

    #[test]
    fn test_convert_64_to_32() {
        let id = convert_steam_id64_to_32(76561198012952267u64);
        assert_eq!(52686539u32, id);
    }
}
