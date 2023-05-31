use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum InitFlags {
    Latency,
    SixteenBit,
    Mono,
    Stereo,
    NoSpeaker,
    Frequency,

    DirectSound,
    AudioTrack,
    DMix,
    // Software,
    // ReInitialize
}

crate::__impl_BassFlags!(InitFlags, [
    (BASS_DEVICE_LATENCY, InitFlags::Latency),
    (BASS_DEVICE_16BITS, InitFlags::SixteenBit),
    (BASS_DEVICE_MONO, InitFlags::Mono),
    (BASS_DEVICE_STEREO, InitFlags::Stereo),
    (BASS_DEVICE_NOSPEAKER, InitFlags::NoSpeaker),
    (BASS_DEVICE_FREQ, InitFlags::Frequency),
    (BASS_DEVICE_DSOUND, InitFlags::DirectSound),
    (BASS_DEVICE_AUDIOTRACK, InitFlags::AudioTrack),
    (BASS_DEVICE_DMIX, InitFlags::DMix),
    // (BASS_DEVICE_SOFTWARE, InitFlags::Software),
    // (BASS_DEVICE_REINIT, InitFlags::ReInitialize)
]);