
pub(crate) trait IBool {
    fn ibool(&self) -> i32;
}
impl IBool for bool {
    fn ibool(&self) -> i32 {
        if *self {1} else {0}
    }
}

/// helper for lengths/positions
pub trait FromLenExt<T> {
    fn from_len(self) -> T;
}
pub trait FromLen {
    fn from_len(t: impl IntoLen) -> Self;
}
/// helper for lengths/positions
pub trait IntoLen {
    fn into_len(&self) -> u64;
}

macro_rules! __impl_len {
    ($($type:ty),+) => {
        $(
            impl IntoLen for $type {
                fn into_len(&self) -> u64 {
                    *self as u64
                }
            }

            impl FromLen for $type {
                fn from_len(t:impl IntoLen) -> Self {
                    t.into_len() as Self
                }
            }
        )+
    };
}
__impl_len!(
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize
);

impl<T:FromLen, U:IntoLen> FromLenExt<T> for U {
    fn from_len(self) -> T {
        T::from_len(self)
    }
}