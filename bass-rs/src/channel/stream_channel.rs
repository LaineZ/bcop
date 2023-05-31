use std::ops::Deref;
use std::ptr::{null, null_mut};
use std::sync::Arc;

use crate::prelude::*;

/// ## Stream channel. 
/// 
/// Use this if you want to play audio from memory
/// 
/// See [`here`](https://www.un4seen.com/doc/#bass/BASS_StreamCreate.html) for more information
/// 
/// # Usage
/// 
/// Create a new stream channel with [`StreamChannel::create_from_memory`](#method.create_from_memory)
/// 
/// See [`Channel`] for further documentation
/// 
/// # Dropping
/// See [`Channel`] for drop behaviour
#[derive(Clone)]
pub struct StreamChannel {
    pub channel: Channel,

    /// needed so the data stays in memory while its needed by bass
    _data: Arc<Vec<u8>>
}
impl StreamChannel {
    /// Create a StreamChannel from bytes in memory
    /// ```ignore
    /// let bytes = std::fs::read(path.as_ref())?;
    /// let channel = StreamChannel::load_from_memory(bytes, 0i32).expect("Error creating stream channel")
    /// channel.play().expect("error playing channel");
    /// ```
    pub fn load_from_memory(bytes: Vec<u8>, offset: impl IntoLen) -> BassResult<Self> {
        // create the stream
        let handle = bass_sys::BASS_StreamCreateFile(
            true.ibool(),
            bytes.as_ptr() as *const c_void,
            offset.into_len(),
            bytes.len() as u64,
            BASS_STREAM_PRESCAN
        );
        // check for an error when creating the stream
        check_bass_err!(handle);

        // double check the channel is valid
        check_bass_err!(bass_sys::BASS_ChannelGetInfo(handle, &mut new_channel_info()));

        // should be good to go from here
        #[cfg(feature="drop_debug")] 
        println!("created stream channel id: {}", handle);
        Ok(Self {
            channel: Channel::new(handle),
            _data: Arc::new(bytes)
        })
    }

    /// Create a StreamChannel from a path
    /// ```ignore
    /// let bytes = "path_to_file";
    /// let channel = StreamChannel::load_from_path(path, 0i32).expect("Error creating stream channel")
    /// channel.play().expect("error playing channel");
    /// ```
    pub fn load_from_path(path: impl AsRef<str>, offset: impl IntoLen) -> BassResult<Self> {
        let path = path.as_ref();
        // create the stream
        let handle = bass_sys::BASS_StreamCreateFile(
            false.ibool(),
            path.as_ptr() as *const c_void,
            offset.into_len(),
            0,
            BASS_STREAM_PRESCAN
        );
        // check for an error when creating the stream
        check_bass_err!(handle);

        // double check the channel is valid
        check_bass_err!(bass_sys::BASS_ChannelGetInfo(handle, &mut new_channel_info()));

        // should be good to go from here
        Ok(Self {
            channel: Channel::new(handle),
            _data: Arc::new(Vec::new())
        })
    }

    /// Create a StreamChannel from a URL
    pub fn load_from_url(url: impl AsRef<str>, offset: impl IntoLen) -> BassResult<Self> {
        let url = url.as_ref();
        // create the stream
        let handle = bass_sys::BASS_StreamCreateURL(
            url.as_ptr() as *const i8,
            offset.into_len() as u32,
            BASS_STREAM_PRESCAN,
            null::<DOWNLOADPROC>() as _,
            null_mut() as *mut c_void,
        );
        // check for an error when creating the stream
        check_bass_err!(handle);

        // double check the channel is valid
        check_bass_err!(bass_sys::BASS_ChannelGetInfo(handle, &mut new_channel_info()));

        // should be good to go from here
        Ok(Self {
            channel: Channel::new(handle),
            _data: Arc::new(Vec::new())
        })
    }

    // pub fn create(freq: u64, ) -> BassResult<Self> {
    //     BASS_StreamCreate(freq, channels, flags, )
    // }

    
    /// alias for load_from_memory
    /// maintains backwards compatability
    #[deprecated]
    pub fn create_from_memory(bytes: Vec<u8>, offset: impl IntoLen) -> BassResult<Self> {
        Self::load_from_memory(bytes, offset)
    }

    /// alias for load_from_path
    /// maintains backwards compatability
    #[deprecated]
    pub fn create_from_path(path: impl AsRef<str>, offset: impl IntoLen) -> BassResult<Self> {
        Self::load_from_path(path, offset)
    }
}
impl Deref for StreamChannel {
    type Target = Channel;

    fn deref(&self) -> &Self::Target {
        &self.channel
    }
}
impl Drop for StreamChannel {
    fn drop(&mut self) {
        let count = Arc::strong_count(&self.handle);
        if count == 1 {
            #[cfg(feature="drop_debug")] 
            println!("dropping stream channel id: {}", self.channel.handle);

            // need to free the bass channel
            if BASS_StreamFree(*self.channel.handle) == 0 {
                panic!("error dropping stream: {:?}", BassError::get_last_error())
            }
        }
    }
}

#[inline]
fn new_channel_info() -> BassChannelInfo {
    BassChannelInfo::new(
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        vec![0i8].as_ptr()
    )
}