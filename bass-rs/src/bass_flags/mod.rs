mod init_flags;
mod device_flags;

pub use init_flags::*;
pub use device_flags::*;


pub trait ToBassFlags<T> {
    fn to_flags(&self) -> Vec<T>;
}
pub trait AsBassFlags {
    fn to_num(self) -> u32;
}

#[macro_export]
macro_rules! __impl_BassFlags {
    ($type:ty, $items:expr) => {
        impl ToBassFlags<$type> for u32 {
            fn to_flags(&self) -> Vec<$type> {

                let mut list = Vec::new();
                for (bass_flag, enum_value) in $items {
                    if (self & bass_flag) > 0 {
                        list.push(enum_value);
                    }
                }
                
                list
            }
        }

        impl AsBassFlags for Vec<$type> {
            fn to_num(self) -> u32 {
                let mut total = 0;

                for (flag, value) in $items {
                    if self.contains(&value) {
                        total |= flag
                    }
                }

                total
            }
        }

    };
}