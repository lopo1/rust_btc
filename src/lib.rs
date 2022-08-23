mod error;
mod utils;
mod blocks;
mod storage;
mod transactions;
mod wallets;
mod networks;

pub use blocks::*;
pub use storage::*;
pub use transactions::*;
pub use wallets::*;
pub use networks::*;

pub fn add(left: usize, right: usize) -> usize {
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
