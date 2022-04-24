extern crate core;

// pub mod value;
// pub mod typing;
// pub mod env;
// pub mod ast;
// pub mod parser;
// pub mod eval;
// pub mod function;
// pub mod definition;
// pub mod storage;
// pub mod mutation;
// pub mod plan;
pub mod relation;
pub mod db;
pub mod error;
pub mod parser;

#[cfg(test)]
mod tests {
    #[test]
    fn import() {
        use cozorocks::*;
        let _o = OptionsPtr::default();
        println!("Hello");
    }
}