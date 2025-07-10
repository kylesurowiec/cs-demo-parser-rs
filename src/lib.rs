pub mod bitreader;
pub mod commands;
pub mod common;
pub mod constants;
pub mod dispatcher;
pub mod events;
pub mod game_events;
pub mod game_rules;
pub mod game_state;
pub mod gamerules;
pub mod match_info;
pub mod matchinfo;
pub mod parser;
pub use proto;
pub mod sendtables;
pub mod sendtables1;
pub mod sendtables2;
pub mod stringtables;
pub mod utils;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
