#![allow(unused_variables, dead_code)]
use once_cell::sync::Lazy;
use std::{collections::HashMap, sync::Arc};

use crate::prelude::*;

/// ## Bass Channel
/// 
/// This is the underlying Channel object for [`StreamChannel`], [`SampleChannel`] and [`MusicChannel`]
/// 
/// It is not recommended to create this object manually, as it requires a `handle` to the underlying bass stream.
/// 
/// # Dropping
/// 
/// This object is clone-able, and the underlying channel will be preserved.
/// 
/// Once the final reference has been dropped, the underlying channel will be freed
#[derive(Clone, PartialEq)]
pub struct Channel {
    // probably shouldnt be pub but whatever
    pub handle: Arc<u32>,
    pub default_frequency: f32,
}
impl Channel {
    /// Create a new channel, where `handle` is the handle of the underlying bass channel
    pub fn new(handle: u32) -> Self {
        let default_frequency = if handle != 0 {
            let mut value = 0.0;
            BASS_ChannelGetAttribute(handle, ChannelAttribute::Frequency.into(), &mut value);
            value
        } else {
            0.0
        };

        Self {
            handle: Arc::new(handle),
            default_frequency
        }
    }

    /// Get a channel attribute
    /// 
    /// Returns an error if there was a problem getting the attribute
    pub fn get_attribute(&self, attrib: ChannelAttribute) -> BassResult<f32> {
        let mut value = 0.0;
        check_bass_err!(BASS_ChannelGetAttribute(*self.handle, attrib.into(), &mut value));
        Ok(value)
    }
    /// Set a channel attribute
    /// 
    /// Returns an error if there was a problem setting the attribute (ie value is out of bounds for attribute)
    pub fn set_attribute(&self, attrib: ChannelAttribute, value: f32) -> BassResult<()> {
        check_bass_err!(BASS_ChannelSetAttribute(*self.handle, attrib.into(), value));
        Ok(())
    }

    /// Get the length of the channel in bytes
    /// 
    /// Returns an error if the length is not avaiable
    pub fn get_length(&self) -> BassResult<u64> {
        Ok(check_bass_err!(BASS_ChannelGetLength(*self.handle, BASS_POS_BYTE)))
    }

    /// Get the length of the channel in seconds
    /// 
    /// Returns an error if the length is not avaiable
    pub fn get_length_seconds(&self) -> BassResult<f64> {
        let len_bytes = check_bass_err!(BASS_ChannelGetLength(*self.handle, BASS_POS_BYTE));
        Self::bytes2seconds(&self, len_bytes)
    }

    /// get the position of the audio in ms
    /// 
    /// returns an error if there was a problem getting the position
    pub fn get_position(&self) -> BassResult<f64> {
        let pos = check_bass_err_val!(BASS_ChannelGetPosition(*self.handle, BASS_POS_BYTE), u64::MAX);
        let secs = self.bytes2seconds(pos)? * 1000.0;
        Ok(secs)
    }

    /// Set the position of the audio in ms
    /// 
    /// Returns an error if there was a problem setting the position (ie ms > stream length)
    pub fn set_position(&self, ms:f64) -> BassResult<()> {
        let pos = self.seconds2bytes(ms/1000.0)?.into_len();
        check_bass_err!(BASS_ChannelSetPosition(*self.handle, pos, BASS_POS_BYTE));
        Ok(())
    }

    /// Get the time at the byte index, in seconds
    /// 
    /// Returns an error if there was a problem with the conversion (ie pos > len)
    /// 
    /// You probably dont need this
    pub fn bytes2seconds(&self, pos: impl IntoLen) -> BassResult<f64> {
        let secs = BASS_ChannelBytes2Seconds(*self.handle, pos.into_len());
        check_bass_err_bool!(secs < 0.0);
        Ok(secs)
    }
    
    /// Get the byte index at the current time in seconds
    /// 
    /// Returns an error if there was a problem with the conversion (ie secs > len)
    /// 
    /// You probably dont need this
    pub fn seconds2bytes(&self, secs: f64) -> BassResult<impl IntoLen> {
        Ok(check_bass_err_val!(BASS_ChannelSeconds2Bytes(*self.handle, secs), u64::MAX))
    }

    /// Play the audio, with the option to restart or not
    /// 
    /// Returns an error if there was an issue playing the audio
    pub fn play(&self, restart:bool) -> BassResult<()> {
        check_bass_err!(BASS_ChannelPlay(*self.handle, restart.ibool()));
        Ok(())
    }

    /// Pause the audio
    /// 
    /// Returns an error if there was an issue pausing the audio
    pub fn pause(&self) -> BassResult<()> {
        check_bass_err!(BASS_ChannelPause(*self.handle));
        Ok(())
    }
    
    /// Stop the audio
    /// 
    /// Returns an error if there was an issue stopping the audio
    pub fn stop(&self) -> BassResult<()> {
        check_bass_err!(BASS_ChannelStop(*self.handle));
        Ok(())
    }

    /// Get the [`PlaybackState`] of the channel
    /// 
    /// Returns an error if there was an issue getting the state
    pub fn get_playback_state(&self) -> BassResult<PlaybackState> {
        let val:PlaybackState = BASS_ChannelIsActive(*self.handle).into();

        // if the val is `stopped` it may be an error
        if let PlaybackState::Stopped = val {
            match BassError::from_code(bass_sys::BASS_ErrorGetCode()) {
                BassError::Ok => {} // not an error, channel is just stopped
                err => return Err(err)
            }
        }

        Ok(val)
    }

    /// Get a list of data for this channel.
    /// 
    /// For example, you can get an FFT data list for the current sample data being played
    /// 
    /// See [`Here`](https://www.un4seen.com/doc/#bass/BASS_ChannelGetData.html) for more info
    pub fn get_data(&self, mode: DataType, length:impl IntoLen) -> BassResult<Vec<f32>> {
        let mut data:Vec<f32> = Vec::with_capacity(length.into_len() as usize);
        // fill in data
        for _ in 0..length.into_len() {data.push(0.0)}
        check_bass_err_val!(BASS_ChannelGetData(*self.handle, data.as_mut_ptr() as *mut c_void, mode.into()), u32::MAX);
        Ok(data)
    }


    // convenience functions

    /// Get the volume for this channel
    /// 
    /// Alias for 
    /// ```ignore
    /// self.get_attribute(Volume)
    /// ```
    pub fn get_volume(&self) -> BassResult<f32> {
        self.get_attribute(ChannelAttribute::Volume)
    }
    
    /// Set the volume for this channel
    /// 
    /// Alias for 
    /// ```ignore
    /// self.set_attribute(Volume, vol)
    /// ```
    pub fn set_volume(&self, vol: f32) -> BassResult<()> {
        self.set_attribute(ChannelAttribute::Volume, vol)
    }


    /// Get the volume for this channel
    /// 
    /// Alias for 
    /// ```ignore
    /// self.get_attribute(Frequency) / self.default_frequency
    /// ```
    pub fn get_rate(&self) -> BassResult<f32> {
        Ok(self.get_attribute(ChannelAttribute::Frequency)? / self.default_frequency)
    }

    /// Set the playback rate for this channel
    /// 
    /// Alias for 
    /// ```ignore
    /// self.set_attribute(Frequency, self.default_frequency * rate)
    /// ```
    pub fn set_rate(&self, rate: f32) -> BassResult<()> {
        self.set_attribute(ChannelAttribute::Frequency, self.default_frequency * rate)
    }

    pub fn set_device(&self, device: BassDevice) -> BassResult<()> {
        check_bass_err!(bass_sys::BASS_ChannelSetDevice(*self.handle, device.id));
        Ok(())
    }



}
// #[cfg(feature="drop_debug")]
// impl Drop for Channel {
//     fn drop(&mut self) {
//         let count = Arc::<u32>::strong_count(&self.handle);
//         if count == 1 {
//             println!("channel getting dropped: {}", self.handle)
//         }
//     }
// }


const ERROR_MAP: Lazy<HashMap<u32, PlaybackState>> = Lazy::new(|| {
    use PlaybackState::*;

    HashMap::from([
        (BASS_ACTIVE_STOPPED, Stopped),
        (BASS_ACTIVE_PLAYING, Playing),
        (BASS_ACTIVE_PAUSED, Paused),
        (BASS_ACTIVE_PAUSED_DEVICE, PausedDevice),
        (BASS_ACTIVE_STALLED, Stalled),
    ])
});

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PlaybackState {
    Stopped,
    Playing,
    Paused,
    PausedDevice,
    Stalled
}
impl From<u32> for PlaybackState {
    fn from(i: u32) -> Self {
        match ERROR_MAP.get(&i) {
            Some(&state) => state,
            None => PlaybackState::Stalled
        }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DataType {
    Float,
    Fixed,
    FFT256,
    FFT512,
    FFT1024,
    FFT2048,
    FFT4096,
    FFT8192,
    FFT16384,
    FFT32768,
    FFTComplex,

    FFTIndividual,
    FFTNoWindow,
    FFTNyquist,
    FFTRemoveDC,
    // FFTNoRemove,
    FFTAvailable,
}
impl Into<u32> for DataType {
    fn into(self) -> u32 {
        use DataType::*;
        match self {
            Float => BASS_DATA_FLOAT,
            Fixed => BASS_DATA_FIXED,
            FFT256 => BASS_DATA_FFT256,
            FFT512 => BASS_DATA_FFT512,
            FFT1024 => BASS_DATA_FFT1024,
            FFT2048 => BASS_DATA_FFT2048,
            FFT4096 => BASS_DATA_FFT4096,
            FFT8192 => BASS_DATA_FFT8192,
            FFT16384 => BASS_DATA_FFT16384,
            FFT32768 => BASS_DATA_FFT32768,
            FFTComplex => BASS_DATA_FFT_COMPLEX,
            FFTIndividual => BASS_DATA_FFT_INDIVIDUAL,
            FFTNoWindow => BASS_DATA_FFT_NOWINDOW,
            FFTNyquist => BASS_DATA_FFT_NYQUIST,
            FFTRemoveDC => BASS_DATA_FFT_REMOVEDC,
            // FFTNoRemove => BASS_DATA_NOREMOVE,
            FFTAvailable => BASS_DATA_AVAILABLE,
        }
    }
}