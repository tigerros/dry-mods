//! Compiled examples are in the [repository](https://github.com/tigerros/dry-mods/tree/master/examples).

#![warn(clippy::cargo)]
#![warn(clippy::pedantic)]
#![warn(clippy::style)]
#![warn(clippy::nursery)]
#![no_std]
#![no_main]

/// Flexible and powerful module declarations.
///
/// # Syntax
/// There's three different patterns the macro matches against:
/// - Define modules and use the contents (`::*`).
/// - Define modules.
/// - Use the contents of modules that are already in scope.
/// - Use the contents of modules that are in a root module.
/// For example, `crate::{mod1, mod2}` instead of `crate::mod1, crate::mod2`.
///
/// After each pattern, you can type a semicolon (`;`) and make another one!
///
/// # Examples
/// This showcases each of the three patterns:
/// ```rust,ignore
/// dry_mods::mods! {
///     // `foo` and `bar` will be defined with the `pub(crate)` visibility but the contents will be completely public.
///     pub(crate) mod pub use foo, bar;
///     // `baz` is totally private, but the contents are exposed in the module where `mods!` was called.
///     use baz;
///     // `my_mod` is now a public module.
///     pub mod my_mod;
///     // Use the root module for crate internals.
///     pub(crate) use crate::{int1, int2, int3, int4, int5};
/// }
///
/// // Generates:
///
/// pub(crate) mod foo;
/// pub(crate) mod bar;
/// pub use foo::*;
/// pub use bar::*;
/// use baz::*;
/// pub mod my_mod;
/// pub(crate) crate::{int1::*, int2::*, int3::*, int4::*, int5::*};
/// ```
#[macro_export]
macro_rules! mods {
    () => {};

    ($mod_vis:vis mod $use_vis:vis use $($module:ident),+; $($rest:tt)*) => {
        $($mod_vis mod $module;)+
        $($use_vis use $module::*;)+
        ::dry_mods::mods! { $($rest)* }
    };

    ($mod_vis:vis mod $($module:ident),+; $($rest:tt)*) => {
        $($mod_vis mod $module;)+
        ::dry_mods::mods! { $($rest)* }
    };

    // No root.
    ($use_vis:vis use $($mod_path:path),+; $($rest:tt)*) => {
        $($use_vis use $mod_path::*;)+
        ::dry_mods::mods! { $($rest)* }
    };

    // Yes root.
    ($use_vis:vis use $root:ident::{$($mod_path:path),+}; $($rest:tt)*) => {
        $($use_vis use $root::{$mod_path::*};)+
        ::dry_mods::mods! { $($rest)* }
    };
}

/// Generates a `prelude` module with some uses.
///
/// # Syntax
/// At the start of each pattern, you can add attributes that will then be added to the `prelude`
/// module. You can also write documentation!
///
/// There's three different patterns the macro matches against.
/// "use" means "use in the `mod prelude`".
/// - Define modules in the file and use the contents (`::*`).
/// This will use `super::mod_name` to get the modules in the file.
/// - Use the contents of modules that are already in scope.
/// - Use the contents of modules that are in a root module.
/// For example, `crate::{mod1, mod2}` instead of `crate::mod1, crate::mod2`.
///
/// After each pattern, you can type a semicolon (`;`) and make another one!
///
/// # Examples
/// This showcases each of the three patterns:
/// ```rust,ignore
/// mod internal1 {
///     pub(crate) fn int1() {}
/// }
///
/// mod internal2 {
///     pub(crate) fn int2() {}
/// }
///
/// dry_mods::prelude! {
///     // `foo` and `bar` will be public modules and their contents will also be public.
///     /// Re-exports some commonly used modules.
///     pub mod pub use foo, bar;
///     // `internal1` and `internal2` will only be visible within the crate.
///     // We don't use `mod` here, because they are already defined.
///     pub(crate) use crate::{internal1, internal2};
/// }
///
/// // Generates:
///
/// pub mod foo;
/// pub mod bar;
/// #[doc = " Re-exports some commonly used modules."]
/// pub mod prelude {
///     pub use super::foo::*;
///     pub use super::bar::*;
///     pub(crate) use crate::{internal1::*, internal2::* };
/// }
/// ```
#[macro_export]
macro_rules! prelude {
    () => {};

    // Using modules in the prelude by prepending "U="
    (U=) => {};

    // No use root.
    (U=$vis:vis use $($mod_path:path),+; $($rest:tt)*) => {
        $($vis use $mod_path::*;)+
        ::dry_mods::prelude! { U=$($rest)* }
    };

    // Yes use root.
    (U=$vis:vis use $root:ident::{$($mod_path:path),+}; $($rest:tt)*) => {
        $vis use $root::{$($mod_path::*,)+};
        ::dry_mods::prelude! { U=$($rest)* }
    };

    // Prelude, mod and use.
    ($(#[$attr:meta])* $mod_vis:vis mod $use_vis:vis use $($module:ident),+; $($rest:tt)*) => {
        $($mod_vis mod $module;)*
        $(#[$attr])
        *
        $use_vis mod prelude {
            $($use_vis use super::$module::*;)*
            ::dry_mods::prelude! { U=$($rest)* }
        }
    };

    // Prelude, use without root.
    ($(#[$attr:meta])* $use_vis:vis use $($mod_path:path),+; $($rest:tt)*) => {
        $(#[$attr])
        *
        $use_vis mod prelude {
            $($use_vis use $mod_path::*;)+
            ::dry_mods::prelude! { U=$($rest)* }
        }
    };

    // Prelude, use with root.
    ($(#[$attr:meta])* $use_vis:vis use $root:ident::{$($mod_path:path),+}; $($rest:tt)*) => {
        $(#[$attr])
        *
        $use_vis mod prelude {
            $($use_vis use $root::{$mod_path::*};)+
            ::dry_mods::prelude! { U=$($rest)* }
        }
    };
}
