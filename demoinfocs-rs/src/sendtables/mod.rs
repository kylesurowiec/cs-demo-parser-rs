pub mod entity;
pub mod propdecoder;
pub mod serverclass;
pub mod parser;

pub use entity::{Entity, Property, PropertyValue};
pub use serverclass::{PropertyEntry, ServerClass};
pub use parser::Parser as TablesParser;
pub use parser::Parser as SendTableParser;

pub type ServerClasses = Vec<ServerClass>;
