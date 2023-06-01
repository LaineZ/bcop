#![allow(unused_variables)]
use prelude::*;

// mods
pub mod bass_error;
pub mod bass_flags;
pub mod channel;
pub mod devices;
mod macros;
pub mod prelude;
pub mod traits;

/// The main entrypoint for this lib.
///
/// # Usage
///
/// ## Initialization
///
/// There are using `BassBuilder` struct to build BASS
///
/// The `build()` function will return a Bass object, which should not be dropped as long as you want to use bass.
///
/// ## Playing Audio
///
/// To play audio, you must first make a channel.
/// There are three kinds of channel you can create:
///
/// [`StreamChannel`], [`SampleChannel`], and [`MusicChannel`]. (View these structs for usage and examples).
///
/// Once a channel has been created, you can start playing audio by running .play() on it.
///
/// Each channel is de-ref into a [`Channel`], so see it for more usage
///
/// # Dropping
/// When this object is dropped, it will un-initialize bass (Bass_Free).
///
/// Bass will need to be re-initialized before use, and any channels will have to be remade
pub struct Bass;

pub struct BassBuilder {
    device_id: i32,
    frequency: u32,
    flags: Vec<InitFlags>,
    window_ptr: *mut c_void,
}

impl Default for BassBuilder {
    fn default() -> Self {
        Self {
            device_id: -1,
            frequency: 44100,
            flags: Default::default(),
            window_ptr: std::ptr::null::<c_void>() as *mut c_void,
        }
    }
}

/// Struct that allows building BASS.
/// Example:
/// ```
/// fn main() {
///     // Intializes BASS with default output device
///     let bass = Bass::builder().build();
///     
/// }
/// ```
impl BassBuilder {
    /// Initializes a new builder with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Specifies playback audio device for initialization
    pub fn device(mut self, device: BassDevice) -> BassBuilder {
        self.device_id = device.id as i32;
        self
    }


    /// Same as device, but with device index
    pub fn device_index(mut self, device_id: i32) -> BassBuilder {
        self.device_id = device_id;
        self
    }

    /// Specifies output sample rate for device
    pub fn frequency(mut self, freq: u32) -> BassBuilder {
        self.frequency = freq;
        self
    }

    /// BASS Initialization flags
    pub fn flag(mut self, flag: InitFlags) -> BassBuilder {
        self.flags.push(flag);
        self
    }

    /// The application's main window handle... This is only needed when using DirectSound output.
    pub fn window_ptr<W>(mut self, window_ptr: *mut W) -> BassBuilder {
        self.window_ptr = window_ptr as *mut c_void;
        self
    }

    /// Initializes a bass library
    pub fn build(self) -> BassResult<Bass> {
        let flags = self.flags.to_num();

        check_bass_err!(bass_sys::BASS_Init(
            self.device_id,
            self.frequency,
            flags,
            self.window_ptr as *mut c_void,
            std::ptr::null::<c_void>() as *mut c_void
        ));

        Ok(Bass)
    }
}

impl Bass {
    pub fn builder() -> BassBuilder {
        BassBuilder::default()
    }
}
impl Drop for Bass {
    fn drop(&mut self) {
        #[cfg(drop_debug)]
        println!("dropping bass!!");

        if BASS_Free() == 0 {
            panic!("Bass failed to free: {:?}", BassError::get_last_error())
        }
    }
}
