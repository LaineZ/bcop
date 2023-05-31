
/// check if the param is an error (== 0)
/// if it is, get the error code from bass,
/// and return from the function with said error
#[macro_export]
macro_rules! check_bass_err {
    ($check:expr) => {
        $crate::check_bass_err_val!($check, 0)
    };
}

/// check if the param is equal to an error value
/// if it is, get the error code from bass,
/// and return from the function with said error
#[macro_export]
macro_rules! check_bass_err_val {
    ($check:expr, $err_val:expr) => {
        {{
            let n = $check; // this prevents $check running multiple times if it is not a variable, but a function call
            $crate::check_bass_err_bool!(n == $err_val);
            n
        }}
    };
}

/// check if the param is true
/// if it is, get the error code from bass,
/// and return from the function with said error
#[macro_export]
macro_rules! check_bass_err_bool {
    ($check:expr) => {
        if $check {
            return Err($crate::bass_error::BassError::from_code(bass_sys::BASS_ErrorGetCode()));
        }
    };
}
