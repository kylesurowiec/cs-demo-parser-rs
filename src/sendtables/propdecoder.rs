use std::io::Read;

use bitflags::bitflags;

use super::entity::{FlattenedPropEntry, Property, PropertyValue, Vector};
use crate::bitreader::BitReader;

pub const PROP_TYPE_INT: i32 = 0;
pub const PROP_TYPE_FLOAT: i32 = 1;
pub const PROP_TYPE_VECTOR: i32 = 2;
pub const PROP_TYPE_VECTORXY: i32 = 3;
pub const PROP_TYPE_STRING: i32 = 4;
pub const PROP_TYPE_ARRAY: i32 = 5;
pub const PROP_TYPE_DATATABLE: i32 = 6;
pub const PROP_TYPE_INT64: i32 = 7;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropertyType {
    Int,
    Float,
    Vector,
    VectorXY,
    String,
    Array,
    DataTable,
    Int64,
    Any,
}

impl From<i32> for PropertyType {
    fn from(value: i32) -> Self {
        match value {
            | PROP_TYPE_INT => PropertyType::Int,
            | PROP_TYPE_FLOAT => PropertyType::Float,
            | PROP_TYPE_VECTOR => PropertyType::Vector,
            | PROP_TYPE_VECTORXY => PropertyType::VectorXY,
            | PROP_TYPE_STRING => PropertyType::String,
            | PROP_TYPE_ARRAY => PropertyType::Array,
            | PROP_TYPE_DATATABLE => PropertyType::DataTable,
            | PROP_TYPE_INT64 => PropertyType::Int64,
            | _ => PropertyType::Any,
        }
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, Default)]
    pub struct SendPropertyFlags: u32 {
        const UNSIGNED = 1 << 0;
        const COORD = 1 << 1;
        const NOSCALE = 1 << 2;
        const ROUNDDOWN = 1 << 3;
        const ROUNDUP = 1 << 4;
        const NORMAL = 1 << 5;
        const EXCLUDE = 1 << 6;
        const XYZE = 1 << 7;
        const INSIDEARRAY = 1 << 8;
        const PROXYALWAYSYES = 1 << 9;
        const ISVECTORELEM = 1 << 10;
        const COLLAPSIBLE = 1 << 11;
        const COORDMP = 1 << 12;
        const COORDMPLOWPRECISION = 1 << 13;
        const COORDMPINTEGRAL = 1 << 14;
        const CELLCORD = 1 << 15;
        const CELLCORDLOWPRECISION = 1 << 16;
        const CELLCORDINTEGRAL = 1 << 17;
        const CHANGESOFTEN = 1 << 18;
        const VARINT = 1 << 19;
    }
}

#[derive(Debug, Clone, Default)]
pub struct SendTableProperty {
    pub flags: SendPropertyFlags,
    pub low_value: f32,
    pub high_value: f32,
    pub number_of_bits: u32,
    pub number_of_elements: i32,
    pub priority: i32,
    pub raw_type: i32,
}

pub struct PropertyDecoder;

impl PropertyDecoder {
    pub fn decode_prop<R: Read>(&self, prop: &mut Property, reader: &mut BitReader<R>) {
        match prop.entry.prop.raw_type {
            | PROP_TYPE_FLOAT => {
                prop.value.float_val = self.decode_float(&prop.entry.prop, reader);
            },
            | PROP_TYPE_INT => {
                prop.value.int_val = self.decode_int(&prop.entry.prop, reader);
            },
            | PROP_TYPE_VECTORXY => {
                prop.value.vector_val = self.decode_vector_xy(&prop.entry.prop, reader);
            },
            | PROP_TYPE_VECTOR => {
                prop.value.vector_val = self.decode_vector(&prop.entry.prop, reader);
            },
            | PROP_TYPE_ARRAY => {
                prop.value.array_val = self.decode_array(&prop.entry, reader);
            },
            | PROP_TYPE_STRING => {
                prop.value.string_val = self.decode_string(reader);
            },
            | PROP_TYPE_INT64 => {
                prop.value.int64_val = self.decode_int64(&prop.entry.prop, reader);
            },
            | _ => panic!("Unknown prop type {}", prop.entry.prop.raw_type),
        }
    }

    fn decode_int<R: Read>(&self, prop: &SendTableProperty, reader: &mut BitReader<R>) -> i32 {
        if prop.flags.contains(SendPropertyFlags::VARINT) {
            if prop.flags.contains(SendPropertyFlags::UNSIGNED) {
                return reader.read_varint32() as i32;
            }
            return reader.read_signed_varint32() as i32;
        }
        if prop.flags.contains(SendPropertyFlags::UNSIGNED) {
            reader.read_int(prop.number_of_bits) as i32
        } else {
            reader.read_signed_int(prop.number_of_bits) as i32
        }
    }

    fn decode_int64<R: Read>(&self, prop: &SendTableProperty, reader: &mut BitReader<R>) -> i64 {
        if prop.flags.contains(SendPropertyFlags::VARINT) {
            if prop.flags.contains(SendPropertyFlags::UNSIGNED) {
                return reader.read_varint64() as i64;
            }
            return reader.read_signed_varint64();
        }
        let high;
        let low;
        let mut negative = false;
        if prop.flags.contains(SendPropertyFlags::UNSIGNED) {
            low = reader.read_int(32);
            high = reader.read_int(prop.number_of_bits - 32);
        } else {
            negative = reader.read_bit();
            low = reader.read_int(32);
            high = reader.read_int(prop.number_of_bits - 32 - 1);
        }
        let mut result = ((high as i64) << 32) | (low as i64);
        if negative {
            result = -result;
        }
        result
    }

    fn decode_float<R: Read>(&self, prop: &SendTableProperty, reader: &mut BitReader<R>) -> f32 {
        if !(prop.flags & special_float_flags()).is_empty() {
            return self.decode_special_float(prop, reader);
        }
        let dw_interp = reader.read_int(prop.number_of_bits);
        prop.low_value
            + ((prop.high_value - prop.low_value)
                * (dw_interp as f32 / ((1i32 << prop.number_of_bits) - 1) as f32))
    }

    fn decode_special_float<R: Read>(
        &self,
        prop: &SendTableProperty,
        reader: &mut BitReader<R>,
    ) -> f32 {
        if prop.flags.contains(SendPropertyFlags::COORD) {
            return self.read_bit_coord(reader);
        } else if prop.flags.contains(SendPropertyFlags::COORDMP) {
            return self.read_bit_coord_mp(reader, false, false);
        } else if prop.flags.contains(SendPropertyFlags::COORDMPLOWPRECISION) {
            return self.read_bit_coord_mp(reader, false, true);
        } else if prop.flags.contains(SendPropertyFlags::COORDMPINTEGRAL) {
            return self.read_bit_coord_mp(reader, true, false);
        } else if prop.flags.contains(SendPropertyFlags::NOSCALE) {
            return reader.read_float();
        } else if prop.flags.contains(SendPropertyFlags::NORMAL) {
            return self.read_bit_normal(reader);
        } else if prop.flags.contains(SendPropertyFlags::CELLCORD) {
            return self.read_bit_cell_coord(reader, prop.number_of_bits as i32, false, false);
        } else if prop.flags.contains(SendPropertyFlags::CELLCORDLOWPRECISION) {
            return self.read_bit_cell_coord(reader, prop.number_of_bits as i32, true, false);
        } else if prop.flags.contains(SendPropertyFlags::CELLCORDINTEGRAL) {
            return self.read_bit_cell_coord(reader, prop.number_of_bits as i32, false, true);
        }
        panic!("Unexpected special float flag");
    }

    fn read_bit_coord<R: Read>(&self, reader: &mut BitReader<R>) -> f32 {
        let mut int_val = reader.read_int(1) as i32;
        let mut fract_val = reader.read_int(1) as i32;
        let mut res = 0f32;
        let mut negative = false;
        if int_val != 0 || fract_val != 0 {
            negative = reader.read_bit();
            if int_val == 1 {
                int_val = reader.read_int(COORD_INTEGER_BITS) as i32 + 1;
            }
            if fract_val == 1 {
                fract_val = reader.read_int(COORD_FRACTIONAL_BITS_MP) as i32;
            }
            res = int_val as f32 + (fract_val as f32 * COORD_RESOLUTION);
        }
        if negative {
            res = -res;
        }
        res
    }

    fn read_bit_coord_mp<R: Read>(
        &self,
        reader: &mut BitReader<R>,
        is_integral: bool,
        is_low_precision: bool,
    ) -> f32 {
        let mut res = 0f32;
        let mut negative = false;
        let in_bounds = reader.read_bit();
        if is_integral {
            if reader.read_bit() {
                negative = reader.read_bit();
                if in_bounds {
                    res = reader.read_int(COORD_INTEGER_BITS_MP) as f32 + 1.0;
                } else {
                    res = reader.read_int(COORD_INTEGER_BITS) as f32 + 1.0;
                }
            }
        } else {
            let read_int_val = reader.read_bit();
            negative = reader.read_bit();
            let mut int_val = 0i32;
            if read_int_val {
                if in_bounds {
                    int_val = reader.read_int(COORD_INTEGER_BITS_MP) as i32 + 1;
                } else {
                    int_val = reader.read_int(COORD_INTEGER_BITS) as i32 + 1;
                }
            }
            if is_low_precision {
                res = int_val as f32
                    + (reader.read_int(COORD_FRACTIONAL_BITS_MP_LOW_PRECISION) as f32
                        * COORD_RESOLUTION_LOW_PRECISION);
            } else {
                res = int_val as f32
                    + (reader.read_int(COORD_FRACTIONAL_BITS_MP) as f32 * COORD_RESOLUTION);
            }
        }
        if negative {
            res = -res;
        }
        res
    }

    fn read_bit_normal<R: Read>(&self, reader: &mut BitReader<R>) -> f32 {
        let negative = reader.read_bit();
        let fract_val = reader.read_int(NORMAL_FRACT_BITS) as i32;
        let mut res = fract_val as f32 * NORMAL_RESOLUTION;
        if negative {
            res = -res;
        }
        res
    }

    fn read_bit_cell_coord<R: Read>(
        &self,
        reader: &mut BitReader<R>,
        bits: i32,
        is_low_precision: bool,
        is_integral: bool,
    ) -> f32 {
        if is_integral {
            reader.read_int(bits as u32) as f32
        } else {
            let int_val = reader.read_int(bits as u32) as i32;
            if is_low_precision {
                let fract = reader.read_int(COORD_FRACTIONAL_BITS_MP_LOW_PRECISION) as i32;
                int_val as f32 + (fract as f32 * COORD_RESOLUTION_LOW_PRECISION)
            } else {
                let fract = reader.read_int(COORD_FRACTIONAL_BITS_MP) as i32;
                int_val as f32 + (fract as f32 * COORD_RESOLUTION)
            }
        }
    }

    fn decode_vector<R: Read>(
        &self,
        prop: &SendTableProperty,
        reader: &mut BitReader<R>,
    ) -> Vector {
        let mut res = Vector {
            x: self.decode_float(prop, reader) as f64,
            y: self.decode_float(prop, reader) as f64,
            z: 0.0,
        };
        if !prop.flags.contains(SendPropertyFlags::NORMAL) {
            res.z = self.decode_float(prop, reader) as f64;
        } else {
            let absolute = res.x * res.x + res.y * res.y;
            if absolute < 1.0 {
                res.z = (1.0 - absolute).sqrt();
            }
            if reader.read_bit() {
                res.z = -res.z;
            }
        }
        res
    }

    fn decode_array<R: Read>(
        &self,
        fprop: &FlattenedPropEntry,
        reader: &mut BitReader<R>,
    ) -> Vec<PropertyValue> {
        use std::f64;
        let num_bits = ((fprop.prop.number_of_elements as f64).log2().floor() as u32) + 1;
        let mut res = vec![PropertyValue::default(); reader.read_int(num_bits) as usize];
        if let Some(ref elem) = fprop.array_element_prop {
            for v in &mut res {
                let mut tmp = Property {
                    entry: FlattenedPropEntry {
                        prop: elem.clone(),
                        array_element_prop: None,
                        name: String::new(),
                    },
                    value: PropertyValue::default(),
                };
                self.decode_prop(&mut tmp, reader);
                *v = tmp.value;
            }
        }
        res
    }

    fn decode_string<R: Read>(&self, reader: &mut BitReader<R>) -> String {
        let length = reader.read_int(DATA_TABLE_MAX_STRING_BITS) as usize;
        let length = length.min(DATA_TABLE_MAX_STRING_LENGTH);
        reader.read_c_string(length)
    }

    fn decode_vector_xy<R: Read>(
        &self,
        prop: &SendTableProperty,
        reader: &mut BitReader<R>,
    ) -> Vector {
        Vector {
            x: self.decode_float(prop, reader) as f64,
            y: self.decode_float(prop, reader) as f64,
            z: 0.0,
        }
    }
}

const COORD_FRACTIONAL_BITS_MP: u32 = 5;
const COORD_FRACTIONAL_BITS_MP_LOW_PRECISION: u32 = 3;
const COORD_DENOMINATOR: f32 = (1 << COORD_FRACTIONAL_BITS_MP) as f32;
const COORD_RESOLUTION: f32 = 1.0 / COORD_DENOMINATOR;
const COORD_DENOMINATOR_LOW_PRECISION: f32 = (1 << COORD_FRACTIONAL_BITS_MP_LOW_PRECISION) as f32;
const COORD_RESOLUTION_LOW_PRECISION: f32 = 1.0 / COORD_DENOMINATOR_LOW_PRECISION;
const COORD_INTEGER_BITS_MP: u32 = 11;
const COORD_INTEGER_BITS: u32 = 14;

const NORMAL_FRACT_BITS: u32 = 11;
const NORMAL_DENOMINATOR: f32 = (1 << (NORMAL_FRACT_BITS - 1)) as f32;
const NORMAL_RESOLUTION: f32 = 1.0 / NORMAL_DENOMINATOR;

const DATA_TABLE_MAX_STRING_BITS: u32 = 9;
const DATA_TABLE_MAX_STRING_LENGTH: usize = 1 << DATA_TABLE_MAX_STRING_BITS;

fn special_float_flags() -> SendPropertyFlags {
    SendPropertyFlags::NOSCALE
        | SendPropertyFlags::COORD
        | SendPropertyFlags::CELLCORD
        | SendPropertyFlags::NORMAL
        | SendPropertyFlags::COORDMP
        | SendPropertyFlags::COORDMPLOWPRECISION
        | SendPropertyFlags::COORDMPINTEGRAL
        | SendPropertyFlags::CELLCORDLOWPRECISION
        | SendPropertyFlags::CELLCORDINTEGRAL
}
