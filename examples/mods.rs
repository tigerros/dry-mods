mod baz {
    pub(crate) fn baz() {
        println!("baz");
    }
}

dry_mods::mods! {
    // `foo` and `bar` will be defined with the `pub(crate)` visibility but the contents will be completely public.
    pub(crate) mod pub use foo, bar;
    // `baz` is totally private, but the contents are exposed in the module where `mods!` was called.
    // We don't use `mod` here, because `baz` is already defined.
    use baz;
}

fn main() {
    foo();
    bar();
    baz();
}
