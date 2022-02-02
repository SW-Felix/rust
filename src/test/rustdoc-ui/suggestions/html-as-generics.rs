// run-rustfix
#![deny(rustdoc::invalid_html_tags)]

/// This Vec<i32> thing!
//~^ERROR unclosed HTML tag `i32`
//~|HELP try marking as source
//~|SUGGESTION `Vec<i32>`
pub struct Generic;

/// This vec::Vec<i32> thing!
//~^ERROR unclosed HTML tag `i32`
//~|HELP try marking as source
//~|SUGGESTION `vec::Vec<i32>`
pub struct GenericPath;

/// This i32<i32> thing!
//~^ERROR unclosed HTML tag `i32`
//~|HELP try marking as source
//~|SUGGESTION `i32<i32>`
pub struct PathsCanContainTrailingNumbers;

/// This Vec::<i32> thing!
//~^ERROR unclosed HTML tag `i32`
//~|HELP try marking as source
//~|SUGGESTION `Vec::<i32>`
pub struct Turbofish;

/// This [link](https://rust-lang.org)::<i32> thing!
//~^ERROR unclosed HTML tag `i32`
//~|HELP try marking as source
//~|SUGGESTION `::<i32>`
pub struct BareTurbofish;

/// This <span>Vec::<i32></span> thing!
//~^ERROR unclosed HTML tag `i32`
//~|HELP try marking as source
//~|SUGGESTION `Vec::<i32>`
pub struct Nested;
