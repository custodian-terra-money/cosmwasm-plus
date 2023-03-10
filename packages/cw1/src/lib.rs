pub mod helpers;
pub mod msg;
pub mod query;

pub use crate::helpers::{Cw1CanonicalContract, Cw1Contract};
pub use crate::msg::Cw1ExecuteMsg;
pub use crate::query::{CanExecuteResponse, Cw1QueryMsg};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
