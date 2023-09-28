/// use nest! macro to define structs with possible nested structs inside.
/// # Examples
///
/// Define a struct with simple fields:
///
/// ```
/// use nestify::nest;
/// nest! {
///     struct MyStruct {
///         a: i32,
///         b: f64,
///     }
/// }
/// ```
///
/// Define a struct with nested structs:
///
/// ```
/// use nestify::nest;
/// nest! {
///     struct MyStruct {
///         a: struct Inner {
///             x: i32,
///             y: i32,
///         },
///         b: f64,
///     }
/// }
/// ```
///
/// In this example, `Inner` struct will be defined and then used inside `MyStruct`.
///
/// Define multiple nested structs:
///
/// ```
/// use nestify::nest;
/// nest! {
///     struct MyStruct {
///         a: struct Inner {
///             x: struct EvenDeeper {
///                 field: i32,
///             },
///             y: i32,
///         },
///         b: f64,
///     }
/// }
/// ```
///
/// In this example, both `Inner` and `EvenDeeper` will be defined and then used accordingly inside `MyStruct`.
#[macro_export]
macro_rules! nest {
    // termination case:
    // there are no more tokens left to process (indicated by the empty [ ]),
    // this arm is matched, and it generates the accumulated structs.    |
    (@parse $( struct $name:ident { $($field:ident : $f_type:ty, )* })+ [ ] ) => {
        $(
        struct $name {
            $( $field : $f_type, )*
        }
        )*
    };


    // this arm detects the order of two consecutive structs and swaps them.
    // it does so when the list of fields for the current struct is empty ([ ]).
    // this ensures that the right fields are given to the right structs
    (@parse
    struct $first:ident {
        $($first_field:ident : $first_type:ty, )*
    }
    struct $second:ident {
        $($second_field:ident : $second_type:ty, )*
    }
    $( struct $rest:ident {
        $($rest_field:ident : $rest_type:ty, )*
    } )* [] $([ $($ignore:tt)* ])+) => {
        nest! {@parse
            struct $second {
                $($second_field : $second_type, )*
            }
            $(struct $rest {
                $($rest_field : $rest_type, )*
            })*
            struct $first {
                $($first_field : $first_type, )*
            }
            /* switch order of structs */
            $( [ $($ignore)* ])+
        }
    };

    // this arm handles the case where a new nested struct (without a trailing comma) is being
    // defined inside another struct. It extracts the new structs name and its body and
    // then generates this new struct while updating the parent struct with a field
    // of the new struct type.
    (@parse
        struct $this_s_name:ident { $( $this_f_name:ident : $this_f_type:ty, )* }
        $( struct $prev_s_name:ident { $( $prev_f_name:ident : $prev_f_type:ty, )* } )*

        [ $new_f_name:ident : struct $new_struct_name:ident { $($struct_body:tt)* }]
        $([ $($ignore:tt)* ])*
    ) => {
        nest! { @parse
            struct $new_struct_name {

            }
            struct $this_s_name {
                $( $this_f_name : $this_f_type, )*
                $new_f_name : $new_struct_name,

            }
            $( struct $prev_s_name { $( $prev_f_name : $prev_f_type, )* } )*

            [ $($struct_body)* ] [ ] $([ $($ignore)* ])* /* add a new field tree */

        }
    };

    // this arm is similar to the previous one but handles the case where the new nested struct
    // has a trailing comma. again, it extracts the new structs name and body, generates the
    // new struct, and updates the parent struct accordingly.
    (@parse
        struct $this_s_name:ident { $( $this_f_name:ident : $this_f_type:ty, )* }
        $( struct $prev_s_name:ident { $( $prev_f_name:ident : $prev_f_type:ty, )* } )*

        [ $new_f_name:ident : struct $new_struct_name:ident { $($struct_body:tt)* }, $( $tail:tt )* ]
        $([ $($ignore:tt)* ])*
    ) => {
        nest! { @parse
            struct $new_struct_name {

            }
            struct $this_s_name {
                $( $this_f_name : $this_f_type, )*
                $new_f_name : $new_struct_name,

            }
            $( struct $prev_s_name { $( $prev_f_name : $prev_f_type, )* } )*

            [ $($struct_body)* ] [ $($tail)* ] $([ $($ignore)* ])* /* add a new field tree */

        }
    };

    // this arm handles the scenario where a new field
    // (not of struct type and without a trailing comma) is being added to a struct. it simply
    // appends the new field to the current struct.
    (@parse
           struct $this_s_name:ident { $( $this_f_name:ident : $this_f_type:ty, )* }
        $( struct $prev_s_name:ident { $( $prev_f_name:ident : $prev_f_type:ty, )* } )*

        [ $new_f_name:ident : $new_f_type:ty ] $( [ $($ignore:tt)* ] )*
    ) => {
        nest! { @parse
            struct $this_s_name {
                $( $this_f_name : $this_f_type, )*
                $new_f_name : $new_f_type, /* define our new field*/
            }

            $(
            struct $prev_s_name { $( $prev_f_name : $prev_f_type, )* } /* put back out previous structures */
            )*

            [  ] $([ $($ignore)* ])* /* rest */
        }
    };

    // This arm handles the scenario where a new field (not of struct type but with a trailing comma)
    // is being added to a struct. Again, it appends the new field to the current struct and continues
    // processing any remaining fields.
    (@parse
           struct $this_s_name:ident { $( $this_f_name:ident : $this_f_type:ty, )* }
        $( struct $prev_s_name:ident { $( $prev_f_name:ident : $prev_f_type:ty, )* } )*

        [ $new_f_name:ident : $new_f_type:ty, $( $tail:tt )* ] $( [ $($ignore:tt)* ] )*
    ) => {
        nest! { @parse
            struct $this_s_name {
                $( $this_f_name : $this_f_type, )*
                $new_f_name : $new_f_type, /* define our new field*/
            }

            $(
            struct $prev_s_name { $( $prev_f_name : $prev_f_type, )* } /* put back out previous structures */
            )*

            [ $($tail)* ] $([ $($ignore)* ])* /* rest */
        }
    };

    // user start state
    (struct $name:ident {
        $($body:tt)*
    }) => {
        nest!{@parse struct $name {  } [ $($body)* ] }
    };
}
