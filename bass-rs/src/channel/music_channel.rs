use std::{ops::Deref, sync::Arc};

use crate::prelude::*;

/// ## Music Channel
/// 
/// Note! this is untested and probably unfinished
/// 
/// Use this if you want to use MOD music
/// 
/// see [`Here`](https://www.un4seen.com/doc/#bass/BASS_MusicLoad.html) for more info
/// 
/// See [`Channel`] for further documentation
/// 
/// # Dropping
/// 
/// See [`Channel`] for drop behaviour
#[derive(Clone)]
pub struct MusicChannel {
    channel: Channel,

    /// needed so the data stays in memory while its needed by bass
    _data: Arc<Vec<u8>>
}

// statics
impl MusicChannel {

    /// Load a MOD music file from memory
    pub fn load_from_memory(data: Vec<u8>, offset: impl IntoLen, flags: u32, freq: u32) -> BassResult<Self> {
        let handle = check_bass_err!(BASS_MusicLoad(
            true.ibool(), 
            data.as_ptr() as *const c_void, 
            offset.into_len(), 
            data.len() as u32, 
            flags, 
            freq
        ));
        
        //TODO!: is there more checking we need to do?

        #[cfg(feature="drop_debug")] 
        println!("created music channel id: {}", handle);
        Ok(Self {
            channel: Channel::new(handle),
            _data: Arc::new(data)
        })
    }

    pub fn load_from_path(path: impl AsRef<str>, offset: impl IntoLen, flags: u32, freq: u32) -> BassResult<Self> {
        let handle = check_bass_err!(BASS_MusicLoad(
            false.ibool(), 
            path.as_ref().as_ptr() as *const c_void, 
            offset.into_len(), 
            0, 
            flags, 
            freq
        ));
        
        // TODO!: is there more checking we need to do?
        Ok(Self {
            channel: Channel::new(handle),
            _data: Arc::new(Vec::new())
        })
    }
}

// methods
impl MusicChannel {
    pub fn get_attribute(&self, attrib: MusicAttribute) -> BassResult<f32> {
        let mut value = 0.0;
        check_bass_err!(BASS_ChannelGetAttribute(*self.channel.handle, attrib.into(), &mut value));
        Ok(value)
    }
    pub fn set_attribute(&self, attrib: MusicAttribute, value: f32) -> BassResult<()> {
        check_bass_err!(BASS_ChannelSetAttribute(*self.channel.handle, attrib.into(), value));
        Ok(())
    }
}

// get channel fns on music channel
impl Deref for MusicChannel {
    type Target = Channel;

    fn deref(&self) -> &Self::Target {
        &self.channel
    }
}

// drop so the channel can be freed in bass
impl Drop for MusicChannel {
    fn drop(&mut self) {
        let count = Arc::<u32>::strong_count(&self.handle);
        if count == 1 {
            #[cfg(feature="drop_debug")] 
            println!("dropping music channel id: {}", self.channel.handle);

            // need to free the bass channel
            if BASS_StreamFree(*self.channel.handle) == 0 {
                panic!("error dropping music channel: {:?}", BassError::get_last_error())
            }
        }
    }
}

// music channel attributes
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MusicAttribute {
    /// The amplification level, 0 (min) to 100 (max) rounded down to a whole number.
    Amplify,
    /// The BPM, 1 (min) to 255 (max) rounded down to a whole number.
    Bpm,
    /// The Pan separation level, 0 (min) to 100 (max), 50 = linear rounded down to a whole number.
    PanSeparation,
    /// The Position scaler, 1 (min) to 256 (max) rounded down to a whole number.
    PositionScaler,
    /// The Speed, 0 (min) to 255 (max) rounded down to a whole number.
    Speed,
    /// A channel volume level, 0 (silent) to 1 (full).
    VolumeChannel,
    /// Global volume level.
    VolumeGlobal,
    /// An instrument/sample volume level, 0 (min) to 64 (max, 128 for IT format) rounded down to a whole number.
    VolumeInstrument,

    // default attributes
    Channel(ChannelAttribute)
}
impl Into<MusicAttribute> for ChannelAttribute {
    fn into(self) -> MusicAttribute {
        MusicAttribute::Channel(self)
    }
}
impl Into<u32> for MusicAttribute {
    fn into(self) -> u32 {
        match self {
            MusicAttribute::Amplify => BASS_ATTRIB_MUSIC_AMPLIFY,
            MusicAttribute::Bpm => BASS_ATTRIB_MUSIC_BPM,
            MusicAttribute::PanSeparation => BASS_ATTRIB_MUSIC_PANSEP,
            MusicAttribute::PositionScaler => BASS_ATTRIB_MUSIC_PSCALER,
            MusicAttribute::Speed => BASS_ATTRIB_MUSIC_SPEED,
            MusicAttribute::VolumeChannel => BASS_ATTRIB_MUSIC_VOL_CHAN,
            MusicAttribute::VolumeGlobal => BASS_ATTRIB_MUSIC_VOL_GLOBAL,
            MusicAttribute::VolumeInstrument => BASS_ATTRIB_MUSIC_VOL_INST,
            MusicAttribute::Channel(c) => c.into(),
        }
    }
}
