/// ```
/// use macos_doctest_1_79::Foo;
///
/// let mut bar = Foo(42);
/// assert_eq!(bar.0, 42);
/// ```
pub struct Foo(pub u64);
