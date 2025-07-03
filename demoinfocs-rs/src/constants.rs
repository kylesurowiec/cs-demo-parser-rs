// Automatically generated from pkg/demoinfocs/constants/constants.go
// Constants representing entity handle bit sizes and masks.

pub const ENTITY_HANDLE_SERIAL_NUMBER_BITS: u32 = 10;

pub const MAX_EDICT_BITS: u32 = 11;
pub const ENTITY_HANDLE_INDEX_MASK: u32 = (1 << MAX_EDICT_BITS) - 1;
pub const ENTITY_HANDLE_BITS: u32 = MAX_EDICT_BITS + ENTITY_HANDLE_SERIAL_NUMBER_BITS;
pub const INVALID_ENTITY_HANDLE: u32 = (1 << ENTITY_HANDLE_BITS) - 1;

pub const MAX_EDICT_BITS_SOURCE2: u32 = 14;
pub const ENTITY_HANDLE_INDEX_MASK_SOURCE2: u32 = (1 << MAX_EDICT_BITS_SOURCE2) - 1;
pub const ENTITY_HANDLE_BITS_SOURCE2: u32 = MAX_EDICT_BITS_SOURCE2 + ENTITY_HANDLE_SERIAL_NUMBER_BITS;
pub const INVALID_ENTITY_HANDLE_SOURCE2: u32 = (1 << ENTITY_HANDLE_BITS_SOURCE2) - 1;
