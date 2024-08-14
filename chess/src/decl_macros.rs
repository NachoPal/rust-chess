//! Dclarative Macros
//!
//! Useful macros for assertions
//!
//!

/// Return Err of the expression: `return Err($expression);`.
///
/// Used as `fail!(expression)`.
#[macro_export]
macro_rules! fail {
    ( $y:expr ) => {{
        return Err($y.into());
    }};
}

/// Evaluate `$x:expr` and if not true return `Err($y:expr)`.
///
/// Used as `ensure!(expression_to_ensure, expression_to_return_on_false)`.
#[macro_export]
macro_rules! ensure {
    ( $x:expr, $y:expr $(,)? ) => {{
        if !$x {
            $crate::fail!($y);
        }
    }};
}

/// Panic if an expression doesn't evaluate to `Ok`.
///
/// Used as `assert_ok!(expression_to_assert, expected_ok_expression)`,
/// or `assert_ok!(expression_to_assert)` which would assert against `Ok(())`.
#[macro_export]
macro_rules! assert_ok {
    ( $x:expr $(,)? ) => {
        let is = $x;
        match is {
            Ok(_) => (),
            _ => assert!(false, "Expected Ok(_). Got {:#?}", is),
        }
    };
    ( $x:expr, $y:expr $(,)? ) => {
        assert_eq!($x, Ok($y));
    };
}

/// Assert an expression returns an error specified.
///
/// Used as `assert_err!(expression_to_assert, expected_error_expression)`
#[macro_export]
macro_rules! assert_err {
    ( $x:expr , $y:expr $(,)? ) => {
        assert_eq!($x, Err($y.into()));
    };
}
