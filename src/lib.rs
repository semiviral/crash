#![feature(
    if_let_guard,                   // #51114 <https://github.com/rust-lang/rust/issues/51114>
    let_chains,                     // 53667 <https://github.com/rust-lang/rust/issues/53667>
    type_alias_impl_trait,
    try_trait_v2
)]

extern crate alloc;

pub mod lexer;
pub mod parser;
