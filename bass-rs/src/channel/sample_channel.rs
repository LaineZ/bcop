use std::ops::Deref;
use std::sync::Arc;

use crate::prelude::*;


/// ## Sample channel. 
/// 
/// Use this if you want to play audio multiple times at once
/// 
/// See [`here`](https://www.un4seen.com/doc/#bass/BASS_SampleCreate.html) for more information
/// 
/// # Usage
/// 
/// Create a new sample channel with [`SampleChannel::load_from_memory`](#method.load_from_memory)
/// 
/// See [`Channel`] for further documentation
/// 
/// # Dropping
/// See [`Channel`] for drop behaviour
#[derive(Clone)]
pub struct SampleChannel {
    handle: Arc<u32>,
    channels: Vec<Channel>,
    newest_channel: Channel,
    _data: Arc<Vec<u8>>
}
impl SampleChannel {
    fn new(handle: u32, data: Vec<u8>) -> BassResult<Self> {
        let mut sc = Self {
            handle: Arc::new(handle),
            channels: Vec::new(),
            newest_channel: Channel::new(0),
            _data: Arc::new(data)
        };
        sc.get_channel()?;

        #[cfg(feature="drop_debug")] 
        println!("created sample channel id: {}", handle);
        Ok(sc)
    }

    /// Create a SampleChannel from bytes in memory
    /// ```ignore
    /// let bytes = std::fs::read(path.as_ref())?;
    /// let channel = SampleChannel::load_from_memory(bytes, 0i32, 32).expect("Error creating sample channel")
    /// channel.play().expect("error playing channel");
    /// ```
    pub fn load_from_memory(data: Vec<u8>, offset: impl IntoLen, max_channels: u32) -> BassResult<Self> {
        Self::new(check_bass_err!(BASS_SampleLoad(
            true.ibool(), 
            data.as_ptr() as *const c_void, 
            offset.into_len(), 
            data.len() as u32, 
            max_channels, 
            BASS_SAMPLE_OVER_POS
        )), data)
    }

    /// Create a SampleChannel from a path
    /// ```ignore
    /// let path = "path_to_mp3";
    /// let channel = SampleChannel::load_from_path(path, 0i32, 32).expect("Error creating sample channel")
    /// channel.play().expect("error playing channel");
    /// ```
    pub fn load_from_path(path: impl AsRef<str>, offset: impl IntoLen, max_channels: u32) -> BassResult<Self> {
        let path = path.as_ref();
        Self::new(check_bass_err!(BASS_SampleLoad(
            false.ibool(), 
            path.as_ptr() as *const c_void, 
            offset.into_len(), 
            0, 
            max_channels, 
            BASS_SAMPLE_OVER_POS
        )), Vec::new())
    }

    /// Get the latest underlying channel for this stream channel
    /// 
    /// Returns an error if there was a problem getting the channel
    pub fn get_channel(&mut self) -> BassResult<Channel> {
        self.newest_channel = Channel::new(check_bass_err!(BASS_SampleGetChannel(*self.handle, false.ibool() as u32)));
        if !self.channels.contains(&self.newest_channel) {
            self.channels.push(self.newest_channel.clone());
        }
        Ok(self.newest_channel.clone())
    }


    /// alias for load_from_memory
    /// maintains backwards compatability
    #[deprecated]
    pub fn create_from_memory(data: Vec<u8>, offset: impl IntoLen, max_channels: u32) -> BassResult<Self> {
        Self::load_from_memory(data, offset, max_channels)
    }

    /// alias for load_from_path
    /// maintains backwards compatability
    #[deprecated]
    pub fn create_from_path(path: impl AsRef<str>, offset: impl IntoLen, max_channels: u32) -> BassResult<Self> {
        Self::load_from_path(path, offset, max_channels)
    }
    /// for debugging
    pub fn get_channels(&self) -> &Vec<Channel> {
        &self.channels
    }

}
impl Drop for SampleChannel {
    fn drop(&mut self) {
        let count = Arc::<u32>::strong_count(&self.handle);
        if count == 1 {
            #[cfg(feature="drop_debug")] 
            println!("dropping sample channel id: {}", self.handle);

            // need to free the bass channel
            if BASS_SampleFree(*self.handle) == 0 {
                panic!("error dropping sample ")
            }
        }
    }
}

impl Deref for SampleChannel {
    type Target = Channel;

    fn deref(&self) -> &Self::Target {
        &self.newest_channel
    }
}
