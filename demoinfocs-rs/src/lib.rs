pub mod bitreader;
pub mod common;
pub mod constants;
pub mod dispatcher;
pub mod events;
pub mod game_state;
pub mod parser;
pub mod proto;
pub mod sendtables;
pub mod sendtables2;

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
