pub mod bitreader;
pub mod dispatcher;
pub mod parser;
pub mod proto;
pub mod sendtables;
pub mod sendtables2;
pub mod events;

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
