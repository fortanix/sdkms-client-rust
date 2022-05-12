/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/// Serialize bitflags as a set of strings.
///
/// This macro supports the same syntax as the [`bitflags!`](https://docs.rs/bitflags)
/// macro. It will in addition implement `Serialize` and `Deserialize` interpreting
/// the bit flags as a set. Serialization happens as a lists of string flags.
///
/// For example:
///
/// ```ignore
/// bitflags_set! {
///     pub struct TestFlags: u8 {
///         const BIT_0 = 1;
///         const BIT_1 = 2;
///         const BIT_2 = 4;
///     }
/// }
///
/// fn test_set() -> TestFlags {
///     BIT_0 | BIT_2
/// }
/// ```
///
/// The output of `test_set` in JSON representation: `["BIT_0", "BIT_2"]`.
macro_rules! bitflags_set {
    ($(#[$outer:meta])*
        $vis:vis struct $n:ident: $t:ty {
            $(
                $(#[$inner:ident $($args:tt)*])*
                const $f:ident = $v:expr;
            )*
        }
    ) => {
        bitflags! {
            $(#[$outer])*
            $vis struct $n: $t {
                $(
                    $(#[$inner $($args)*])*
                    const $f = $v;
                )*
            }
        }

        // create an unused scope where `use` is acceptable
        impl $n { fn _bitflags_set_unused() {
            use serde::{Serialize, Serializer, Deserialize, Deserializer};
            use serde::ser::SerializeSeq;
            use serde::de::{Visitor, SeqAccess};
            use std::fmt;

            impl Serialize for $n {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                    where S: Serializer
                {
                    let mut seq = serializer.serialize_seq(None)?;
                    $(
                    if self.contains($n::$f) {
                        seq.serialize_element(&Names::$f)?;
                    }
                    )*
                    seq.end()
                }
            }

            impl<'de> Deserialize<'de> for $n {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                    where D: Deserializer<'de>
                {
                    struct SetVisitor;

                    impl<'de> Visitor<'de> for SetVisitor {
                        type Value = $n;
                        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                            formatter.write_str(stringify!($n))
                        }

                        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                            where A: SeqAccess<'de>
                        {
                            let mut flags = $n::empty();

                            while let Some(name) = seq.next_element()? {
                                match name {
                                    $(Names::$f => flags.insert($n::$f),)*
                                }
                            }

                            Ok(flags)
                        }
                    }

                    deserializer.deserialize_seq(SetVisitor)
                }
            }

            #[derive(Serialize, Deserialize)]
            #[serde(rename_all = "UPPERCASE")]
            enum Names {
                $($f),*
            }
        }}
    }
}

macro_rules! singleton_backcompat {
    (
        $(#[doc = $doc:expr])*
        $(#[derive $derives:tt])*
        #[serde(rename_all = "UPPERCASE")]
        pub enum $n:ident { $($v:ident),* $(,)* }
    ) => {
        $(#[doc = $doc])*
        $(#[derive $derives])*
        pub enum $n {
            $($v,)*
        }

        #[allow(bad_style, unused)]
        fn $n() {
            use ::serde;
            use std::result::Result;
            mod normal {
                use serde::{Deserialize, Serialize};
                $(#[derive $derives])*
                #[derive(Serialize, Deserialize)]
                #[serde(rename_all = "UPPERCASE")]
                pub enum $n {
                    $($v,)*
                }
            }
            impl serde::Serialize for $n {
                fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                    (match *self { $($n::$v => normal::$n::$v,)* },).serialize(serializer)
                }
            }
            impl<'de> serde::Deserialize<'de> for $n {
                fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                    Ok(match <(normal::$n,)>::deserialize(deserializer)?.0 {
                        $(normal::$n::$v => $n::$v,)*
                    })
                }
            }
        }
    }
}
