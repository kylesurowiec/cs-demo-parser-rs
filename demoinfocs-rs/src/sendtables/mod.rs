pub mod entity;
pub mod parser;
pub mod propdecoder;
pub mod serverclass;

pub use entity::{Entity, Property, PropertyValue};
pub use parser::Parser as TablesParser;
pub use parser::Parser as SendTableParser;
pub use serverclass::{PropertyEntry, ServerClass};

pub type ServerClasses = Vec<ServerClass>;
