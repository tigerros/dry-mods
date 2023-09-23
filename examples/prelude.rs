mod internal1 {
    pub(crate) fn int1() {
        println!("int1");
    }
}

mod internal2 {
    pub(crate) fn int2() {
        println!("int2");
    }
}

dry_mods::prelude! {
    // `foo` and `bar` will be public modules and their contents will also be public.
    /// Re-exports some commonly used modules.
    pub mod pub use foo, bar;
    // `internal1` and `internal2` will only be visible within the crate.
    // We don't use `mod` here, because they are already defined.
    pub(crate) use crate::{internal1, internal2};
}

fn main() {
    prelude::foo();
    prelude::bar();
    prelude::int1();
    prelude::int2();
}
