#[macro_export(local_inner_macros)]
macro_rules! json {
    ( $($tt:tt)* ) => { json_internal!($($tt)*) };
}

// #[macro_export(local_inner_macros)]
// #[doc(hidden)]
// macro_rules! json_internal {
//     ({ $($tt:tt)+ }) => {{
//     }};

//     (@key ($f:expr) $n:ident ($i:expr) ($($key:tt)*) (: $($rest:tt)*)) => {
//         if $n == $i {
//             $f.deserialize(json_internal!($($key)*))?
//         } else {
//             json_internal!(@skipvalue ($f) $n ($i + 1) () ($($rest)*))
//         }
//     };
//     (@key ($f:expr) $n:ident ($i:expr) ($($key:tt)*) ($tt:tt $($rest:tt)*)) => {
//         json_internal!(@key ($f) $n ($i) ($($key)* $tt) ($($rest)*))
//     };
//     (@skipvalue ($f:expr) $n:ident ($i:expr) () (, $($rest:tt)+)) => {
//         json_internal!(@key ($f) $n ($i + 1) () ($($rest)*))
//     };
//     (@skipvalue ($f:expr) $n:ident ($i:expr) () ($(,)?)) => {
//         if $n == $i + 1 {
//             return Ok(None)
//         } else {
//             return Err(<serde::de::value::Error as serde::de::Error>::custom("foobar"))
//         }
//     };
//     (@skipvalue ($f:expr) $n:ident ($i:expr) () ($tt:tt $($rest:tt)*)) => {
//         json_internal!(@skipvalue ($f) $n ($i) () ($($rest)*))
//     };

//     (@value ($f:expr) $n:ident ($i:expr) ($($value:tt)*) (, $($rest:tt)*)) => {
//         if $n == $i {
//             $f.deserialize(json_internal!($($value)*))?
//         } else {
//             json_internal!(@skipkey ($f) $n ($i + 1) () ($($rest)*))
//         }
//     };
//     (@value ($f:expr) $n:ident ($i:expr) ($($value:tt)*) ($(,)?)) => {
//         if $n == $i {
//             $f.deserialize(json_internal!($($value)*))?
//         } else {
//             return Err(<serde::de::value::Error as serde::de::Error>::custom("foobar"))
//         }
//     };
//     (@value ($f:expr) $n:ident ($i:expr) ($($value:tt)*) ($tt:tt $($rest:tt)*)) => {
//         json_internal!(@value ($f) $n ($i) ($($value)* $tt) ($($rest)*))
//     };
//     (@skipkey ($f:expr) $n:ident ($i:expr) () (: $($rest:tt)+)) => {
//         json_internal!(@value ($f) $n ($i + 1) () ($($rest)*))
//     };
//     (@skipkey ($f:expr) $n:ident ($i:expr) () ($tt:tt $($rest:tt)*)) => {
//         json_internal!(@skipkey ($f) $n ($i) () ($($rest)*))
//     };


//     ($x:literal) => { $crate::Lit($x) };
// }

// Rocket relies on this because they export their own `json!` with a different
// doc comment than ours, and various Rust bugs prevent them from calling our
// `json!` from their `json!` so they call `json_internal!` directly. Check with
// @SergioBenitez before making breaking changes to this macro.
//
// Changes are fine as long as `json_internal!` does not call any new helper
// macros and can still be invoked as `json_internal!($($json)+)`.
#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! json_internal {
    //////////////////////////////////////////////////////////////////////////
    // TT muncher for parsing the inside of an array [...]. Produces a vec![...]
    // of the elements.
    //
    // Must be invoked as: json_internal!(@array [] $($tt)*)
    //////////////////////////////////////////////////////////////////////////

    // Done with trailing comma.
    (@array [$($elems:expr,)*]) => {
        json_internal_vec![$($elems,)*]
    };

    // Done without trailing comma.
    (@array [$($elems:expr),*]) => {
        json_internal_vec![$($elems),*]
    };

    // Next element is `null`.
    (@array [$($elems:expr,)*] null $($rest:tt)*) => {
        json_internal!(@array [$($elems,)* json_internal!(null)] $($rest)*)
    };

    // Next element is `true`.
    (@array [$($elems:expr,)*] true $($rest:tt)*) => {
        json_internal!(@array [$($elems,)* json_internal!(true)] $($rest)*)
    };

    // Next element is `false`.
    (@array [$($elems:expr,)*] false $($rest:tt)*) => {
        json_internal!(@array [$($elems,)* json_internal!(false)] $($rest)*)
    };

    // Next element is an array.
    (@array [$($elems:expr,)*] [$($array:tt)*] $($rest:tt)*) => {
        json_internal!(@array [$($elems,)* json_internal!([$($array)*])] $($rest)*)
    };

    // Next element is a map.
    (@array [$($elems:expr,)*] {$($map:tt)*} $($rest:tt)*) => {
        json_internal!(@array [$($elems,)* json_internal!({$($map)*})] $($rest)*)
    };

    // Next element is an expression followed by comma.
    (@array [$($elems:expr,)*] $next:expr, $($rest:tt)*) => {
        json_internal!(@array [$($elems,)* json_internal!($next),] $($rest)*)
    };

    // Last element is an expression with no trailing comma.
    (@array [$($elems:expr,)*] $last:expr) => {
        json_internal!(@array [$($elems,)* json_internal!($last)])
    };

    // Comma after the most recent element.
    (@array [$($elems:expr),*] , $($rest:tt)*) => {
        json_internal!(@array [$($elems,)*] $($rest)*)
    };

    // Unexpected token after most recent element.
    (@array [$($elems:expr),*] $unexpected:tt $($rest:tt)*) => {
        json_unexpected!($unexpected)
    };

    //////////////////////////////////////////////////////////////////////////
    // TT muncher for parsing the inside of an object {...}. Each entry is
    // inserted into the given map variable.
    //
    // Must be invoked as: json_internal!(@object $map () ($($tt)*) ($($tt)*))
    //
    // We require two copies of the input tokens so that we can match on one
    // copy and trigger errors on the other copy.
    //////////////////////////////////////////////////////////////////////////

    // Done.
    (@objectvalue $seed:ident $n:ident ($i:expr)  () () ()) => {};

    // Insert the current entry followed by trailing comma.
    (@objectvalue $seed:ident $n:ident ($i:expr) [] ($value:expr) , $($rest:tt)*) => {
        if $n == $i {
            $seed.deserialize($value)?
        } else {
            json_internal!(@objectvalue $seed $n ($i + 2) () ($($rest)*) ($($rest)*))
        }
    };

    // Current entry followed by unexpected token.
    (@objectvalue $seed:ident $n:ident ($i:expr) [] ($value:expr) $unexpected:tt $($rest:tt)*) => {
        json_unexpected!($unexpected)
    };

    // Insert the last entry without trailing comma.
    (@objectvalue $seed:ident $n:ident ($i:expr) [] ($value:expr)) => {
        if $n == $i {
            $seed.deserialize($value)?
        } else {
            return Err(<serde::de::value::Error as serde::de::Error>::custom("foobar"))
        }
    };

    // Next value is `null`.
    (@objectvalue $seed:ident $n:ident ($i:expr) ($($key:tt)+) (: null $($rest:tt)*) $copy:tt) => {
        json_internal!(@objectvalue $seed $n ($i) [] (json_internal!(null)) $($rest)*)
    };

    // Next value is `true`.
    (@objectvalue $seed:ident $n:ident ($i:expr) ($($key:tt)+) (: true $($rest:tt)*) $copy:tt) => {
        json_internal!(@objectvalue $seed $n ($i) [] (json_internal!(true)) $($rest)*)
    };

    // Next value is `false`.
    (@objectvalue $seed:ident $n:ident ($i:expr) ($($key:tt)+) (: false $($rest:tt)*) $copy:tt) => {
        json_internal!(@objectvalue $seed $n ($i) [] (json_internal!(false)) $($rest)*)
    };

    // Next value is an array.
    (@objectvalue $seed:ident $n:ident ($i:expr) ($($key:tt)+) (: [$($array:tt)*] $($rest:tt)*) $copy:tt) => {
        json_internal!(@objectvalue $seed $n ($i) [] (json_internal!([$($array)*])) $($rest)*)
    };

    // Next value is a map.
    (@objectvalue $seed:ident $n:ident ($i:expr) ($($key:tt)+) (: {$($map:tt)*} $($rest:tt)*) $copy:tt) => {
        json_internal!(@objectvalue $seed $n ($i) [] (json_internal!({$($map)*})) $($rest)*)
    };

    // Next value is an expression followed by comma.
    (@objectvalue $seed:ident $n:ident ($i:expr) ($($key:tt)+) (: $value:expr , $($rest:tt)*) $copy:tt) => {
        json_internal!(@objectvalue $seed $n ($i) [] (json_internal!($value)) , $($rest)*)
    };

    // Last value is an expression with no trailing comma.
    (@objectvalue $seed:ident $n:ident ($i:expr) ($($key:tt)+) (: $value:expr) $copy:tt) => {
        json_internal!(@objectvalue $seed $n ($i) [] (json_internal!($value)))
    };

    // Missing value for last entry. Trigger a reasonable error message.
    (@objectvalue $seed:ident $n:ident ($i:expr) ($($key:tt)+) (:) $copy:tt) => {
        // "unexpected end of macro invocation"
        json_internal!()
    };

    // Missing colon and value for last entry. Trigger a reasonable error
    // message.
    (@objectvalue $seed:ident $n:ident ($i:expr) ($($key:tt)+) () $copy:tt) => {
        // "unexpected end of macro invocation"
        json_internal!()
    };

    // Misplaced colon. Trigger a reasonable error message.
    (@objectvalue $seed:ident $n:ident ($i:expr) () (: $($rest:tt)*) ($colon:tt $($copy:tt)*)) => {
        // Takes no arguments so "no rules expected the token `:`".
        json_unexpected!($colon)
    };

    // Found a comma inside a key. Trigger a reasonable error message.
    (@objectvalue $seed:ident $n:ident ($i:expr) ($($key:tt)*) (, $($rest:tt)*) ($comma:tt $($copy:tt)*)) => {
        // Takes no arguments so "no rules expected the token `,`".
        json_unexpected!($comma)
    };

    // Key is fully parenthesized. This avoids clippy double_parens false
    // positives because the parenthesization may be necessary here.
    (@objectvalue $seed:ident $n:ident ($i:expr) () (($key:expr) : $($rest:tt)*) $copy:tt) => {
        json_internal!(@objectvalue $seed $n ($i) ($key) (: $($rest)*) (: $($rest)*))
    };

    // Refuse to absorb colon token into key expression.
    (@objectvalue $seed:ident $n:ident ($i:expr) ($($key:tt)*) (: $($unexpected:tt)+) $copy:tt) => {
        json_expect_expr_comma!($($unexpected)+)
    };

    // Munch a token into the current key.
    (@objectvalue $seed:ident $n:ident ($i:expr) ($($key:tt)*) ($tt:tt $($rest:tt)*) $copy:tt) => {
        json_internal!(@objectvalue $seed $n ($i) ($($key)* $tt) ($($rest)*) ($($rest)*))
    };



    ////

    // Done.
    (@objectkey $seed:ident $n:ident ($i:expr)  () () ()) => {};

    // Insert the current entry followed by trailing comma.
    (@objectkey $seed:ident $n:ident ($i:expr) [$($key:tt)+] , $($rest:tt)*) => {
        if $n == $i {
            $seed.deserialize(json_internal!($($key)*))?
        } else {
            json_internal!(@objectkey $seed $n ($i + 2) () ($($rest)*) ($($rest)*))
        }
    };

    // Current entry followed by unexpected token.
    (@objectkey $seed:ident $n:ident ($i:expr) [$($key:tt)+] $unexpected:tt $($rest:tt)*) => {
        json_unexpected!($unexpected)
    };

    // Insert the last entry without trailing comma.
    (@objectkey $seed:ident $n:ident ($i:expr) [$($key:tt)+]) => {
        if $n == $i {
            $seed.deserialize(json_internal!($($key)*))?
        } else if $n == $i + 2 {
            return Ok(None)
        } else {
            return Err(<serde::de::value::Error as serde::de::Error>::custom("foobar"))
        }
    };

    // Next value is `null`.
    (@objectkey $seed:ident $n:ident ($i:expr) ($($key:tt)+) (: null $($rest:tt)*) $copy:tt) => {
        json_internal!(@objectkey $seed $n ($i) [$($key)+] $($rest)*)
    };

    // Next value is `true`.
    (@objectkey $seed:ident $n:ident ($i:expr) ($($key:tt)+) (: true $($rest:tt)*) $copy:tt) => {
        json_internal!(@objectkey $seed $n ($i) [$($key)+] $($rest)*)
    };

    // Next value is `false`.
    (@objectkey $seed:ident $n:ident ($i:expr) ($($key:tt)+) (: false $($rest:tt)*) $copy:tt) => {
        json_internal!(@objectkey $seed $n ($i) [$($key)+] $($rest)*)
    };

    // Next value is an array.
    (@objectkey $seed:ident $n:ident ($i:expr) ($($key:tt)+) (: [$($array:tt)*] $($rest:tt)*) $copy:tt) => {
        json_internal!(@objectkey $seed $n ($i) [$($key)+] $($rest)*)
    };

    // Next value is a map.
    (@objectkey $seed:ident $n:ident ($i:expr) ($($key:tt)+) (: {$($map:tt)*} $($rest:tt)*) $copy:tt) => {
        json_internal!(@objectkey $seed $n ($i) [$($key)+] $($rest)*)
    };

    // Next value is an expression followed by comma.
    (@objectkey $seed:ident $n:ident ($i:expr) ($($key:tt)+) (: $value:expr , $($rest:tt)*) $copy:tt) => {
        json_internal!(@objectkey $seed $n ($i) [$($key)+] , $($rest)*)
    };

    // Last value is an expression with no trailing comma.
    (@objectkey $seed:ident $n:ident ($i:expr) ($($key:tt)+) (: $value:expr) $copy:tt) => {
        json_internal!(@objectkey $seed $n ($i) [$($key)+])
    };

    // Missing value for last entry. Trigger a reasonable error message.
    (@objectkey $seed:ident $n:ident ($i:expr) ($($key:tt)+) (:) $copy:tt) => {
        // "unexpected end of macro invocation"
        json_internal!()
    };

    // Missing colon and value for last entry. Trigger a reasonable error
    // message.
    (@objectkey $seed:ident $n:ident ($i:expr) ($($key:tt)+) () $copy:tt) => {
        // "unexpected end of macro invocation"
        json_internal!()
    };

    // Misplaced colon. Trigger a reasonable error message.
    (@objectkey $seed:ident $n:ident ($i:expr) () (: $($rest:tt)*) ($colon:tt $($copy:tt)*)) => {
        // Takes no arguments so "no rules expected the token `:`".
        json_unexpected!($colon)
    };

    // Found a comma inside a key. Trigger a reasonable error message.
    (@objectkey $seed:ident $n:ident ($i:expr) ($($key:tt)*) (, $($rest:tt)*) ($comma:tt $($copy:tt)*)) => {
        // Takes no arguments so "no rules expected the token `,`".
        json_unexpected!($comma)
    };

    // Key is fully parenthesized. This avoids clippy double_parens false
    // positives because the parenthesization may be necessary here.
    (@objectkey $seed:ident $n:ident ($i:expr) () (($key:expr) : $($rest:tt)*) $copy:tt) => {
        json_internal!(@objectkey $seed $n ($i) ($key) (: $($rest)*) (: $($rest)*))
    };

    // Refuse to absorb colon token into key expression.
    (@objectkey $seed:ident $n:ident ($i:expr) ($($key:tt)*) (: $($unexpected:tt)+) $copy:tt) => {
        json_expect_expr_comma!($($unexpected)+)
    };

    // Munch a token into the current key.
    (@objectkey $seed:ident $n:ident ($i:expr) ($($key:tt)*) ($tt:tt $($rest:tt)*) $copy:tt) => {
        json_internal!(@objectkey $seed $n ($i) ($($key)* $tt) ($($rest)*) ($($rest)*))
    };


    //////////////////////////////////////////////////////////////////////////
    // The main implementation.
    //
    // Must be invoked as: json_internal!($($json)+)
    //////////////////////////////////////////////////////////////////////////

    (null) => {
        $crate::Null
    };

    (true) => {
        $crate::Bool(true)
    };

    (false) => {
        $crate::Bool(false)
    };

    ([]) => {
        $crate::EmptyList
    };

    ([ $($tt:tt)+ ]) => {
        $crate::Array(json_internal!(@array [] $($tt)+))
    };

    ({}) => {
        $crate::EmptyMap
    };

    ({ $($tt:tt)+ }) => {{
        #[derive(Copy, Clone)]
        struct Map;
        struct MapState(usize);

        impl<'de> serde::de::Deserializer<'de> for Map {
            type Error = serde::de::value::Error;

            fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
            where
                V: serde::de::Visitor<'de>,
            {
                visitor.visit_map(MapState(0))
            }

            serde::forward_to_deserialize_any! {
                bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
                bytes byte_buf option unit unit_struct newtype_struct seq tuple
                tuple_struct map struct enum identifier ignored_any
            }
        }

        impl<'de> serde::de::MapAccess<'de> for MapState {
            type Error = serde::de::value::Error;

            fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
            where
                K: serde::de::DeserializeSeed<'de>,
            {
                let n = self.0;
                let res = json_internal!(@objectkey seed n (0) () ($($tt)+) ($($tt)+));
                self.0 += 1;
                Ok(Some(res))
            }
            fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
            where
                V: serde::de::DeserializeSeed<'de>,
            {
                let n = self.0;
                let res = json_internal!(@objectvalue seed n (1) () ($($tt)+) ($($tt)+));
                self.0 += 1;
                Ok(res)
            }
        }

        Map
    }};

    // Any Serialize type: numbers, strings, struct literals, variables etc.
    // Must be below every other rule.
    ($other:expr) => {
        $crate::Lit($other)
    };
}

// The json_internal macro above cannot invoke vec directly because it uses
// local_inner_macros. A vec invocation there would resolve to $crate::vec.
// Instead invoke vec here outside of local_inner_macros.
#[macro_export]
#[doc(hidden)]
macro_rules! json_internal_vec {
    ($($content:tt)*) => {
        vec![$($content)*]
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! json_unexpected {
    () => {};
}

#[macro_export]
#[doc(hidden)]
macro_rules! json_expect_expr_comma {
    ($e:expr , $($tt:tt)*) => {};
}
