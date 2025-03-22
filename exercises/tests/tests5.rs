// tests5.rs
//
// An `unsafe` in Rust serves as a contract.
//
// When `unsafe` is marked on an item declaration, such as a function,
// a trait or so on, it declares a contract alongside it. However,
// the content of the contract cannot be expressed only by a single keyword.
// Hence, its your responsibility to manually state it in the `# Safety`
// section of your documentation comment on the item.
//
// When `unsafe` is marked on a code block enclosed by curly braces,
// it declares an observance of some contract, such as the validity of some
// pointer parameter, the ownership of some memory address. However, like
// the text above, you still need to state how the contract is observed in
// the comment on the code block.
//
// NOTE: All the comments are for the readability and the maintainability of
// your code, while the Rust compiler hands its trust of soundness of your
// code to yourself! If you cannot prove the memory safety and soundness of
// your own code, take a step back and use safe code instead!
//
// Execute `rustlings hint tests5` or use the `hint` watch subcommand for a

/// # Safety
///
/// The `address` must contain a mutable reference to a valid `u32` value.
unsafe fn modify_by_address(mut address: usize) {
    // TODO: Fill your safety notice of the code block below to match your
    // code's behavior and the contract of this function. You may use the
    // comment of the test below as your format reference.
    unsafe {
        //todo!("Your code goes here")
        let ptr_ = address as *mut u32;
        *ptr_ = 0xaabbccdd;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut t: u32 = 0x12345678;
        // SAFETY: The address is guaranteed to be valid and contains
        // a unique reference to a `u32` local variable.
        unsafe {
            modify_by_address(&mut t as *mut u32 as usize);
        }
        assert!(t == 0xaabbccdd);
    }
}
// 当在项声明上标记 `unsafe`，例如函数、trait 等时，它同时声明了一个契约。
// 然而，这个契约的内容无法仅通过单个关键字来表达。
// 因此，你有责任在项的文档注释中的 `# Safety` 部分手动说明其内容。
//
// 当在由花括号包围的代码块上标记 `unsafe` 时，
// 它声明了对某个契约的遵守，例如某个指针参数的有效性、某个内存地址的所有权。
// 然而，如上文所述，你仍然需要在代码块的注释中说明如何遵守该契约。
//
// 注意：所有注释都是为了代码的可读性和可维护性，
// 而 Rust 编译器将对你代码的正确性信任交给了你自己！
// 如果你无法证明自己代码的内存安全性和正确性，
// 请退一步，转而使用安全的代码！
