#![allow(unused_variables, dead_code, non_snake_case)]
use std::{collections::HashMap};
use once_cell::sync::Lazy;

pub type BassResult<T> = Result<T, BassError>;
const ERROR_MAP: Lazy<HashMap<i32, BassError>> = Lazy::new(|| {
    use BassError::*;
    use bass_sys::*;

    HashMap::from([
        (BASS_OK, BassError::Ok),
        (BASS_ERROR_MEM, Mem),
        (BASS_ERROR_FILEOPEN, FileOpen),
        (BASS_ERROR_DRIVER, Driver),
        (BASS_ERROR_BUFLOST, BufLost),
        (BASS_ERROR_HANDLE, Handle),
        (BASS_ERROR_FORMAT, Format),
        (BASS_ERROR_POSITION, Position),
        (BASS_ERROR_INIT, Init),
        (BASS_ERROR_START, Start),
        (BASS_ERROR_ALREADY, Already),
        (BASS_ERROR_NOTAUDIO, Notaudio),
        (BASS_ERROR_NOCHAN, Nochan),
        (BASS_ERROR_ILLTYPE, Illtype),
        (BASS_ERROR_ILLPARAM, Illparam),
        (BASS_ERROR_NO3D, No3d),
        (BASS_ERROR_NOEAX, Noeax),
        (BASS_ERROR_DEVICE, Device),
        (BASS_ERROR_NOPLAY, Noplay),
        (BASS_ERROR_FREQ, Freq),
        (BASS_ERROR_NOTFILE, Notfile),
        (BASS_ERROR_NOHW, Nohw),
        (BASS_ERROR_EMPTY, Empty),
        (BASS_ERROR_NONET, Nonet),
        (BASS_ERROR_CREATE, Create),
        (BASS_ERROR_NOFX, Nofx),
        (BASS_ERROR_NOTAVAIL, Notavail),
        (BASS_ERROR_DECODE, Decode),
        (BASS_ERROR_DX, Dx),
        (BASS_ERROR_TIMEOUT, Timeout),
        (BASS_ERROR_FILEFORM, Fileform),
        (BASS_ERROR_SPEAKER, Speaker),
        (BASS_ERROR_VERSION, Version),
        (BASS_ERROR_CODEC, Codec),
        (BASS_ERROR_ENDED, Ended),
        (BASS_ERROR_BUSY, Busy),
    ])
});

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BassError {
    /// all is ok
    Ok,
    /// Memory error
    Mem,
    ///	Can't open the file
    FileOpen,
    /// Can't find a free/valid driver
    Driver,
    /// The sample buffer was lost
    BufLost,
    /// Invalid handle
    Handle,
    /// Unsupported sample format
    Format,
    /// Invalid playback position
    Position,
    /// BASS_Init has not been successfully called
    Init,
    /// BASS_Start has not been successfully called
    Start,

    /// No CD in drive
    Nocd,
    /// Invalid track number
    Cdtrack,
    /// Already initialized/paused/whatever
    Already,
    /// Not paused
    Nopause,
    /// Not an audio track
    Notaudio,
    /// Can't get a free channel
    Nochan,
    /// An illegal type was specified
    Illtype,
    /// An illegal parameter was specified
    Illparam,
    /// No 3D support
    No3d,
    /// No EAX support
    Noeax,
    /// Illegal device number
    Device,
    /// Not playing
    Noplay,
    /// Illegal sample rate
    Freq,
    /// The stream is not a file stream
    Notfile,
    /// No hardware voices available
    Nohw,
    /// The MOD music has no sequence data
    Empty,
    /// No internet connection could be opened
    Nonet,
    /// Couldn't create the file
    Create,
    /// Effects are not available
    Nofx,
    /// The channel is playing
    Playing,
    /// Requested data is not available
    Notavail,
    /// The channel is a 'decoding channel'
    Decode,
    /// A sufficient DirectX version is not installed
    Dx,
    /// Connection timedout
    Timeout,
    /// Unsupported file format
    Fileform,
    /// Unavailable speaker
    Speaker,
    /// Invalid BASS version (used by add-ons)
    Version,
    /// Codec is not available/supported
    Codec,
    /// The channel/file has ended
    Ended,
    /// The device is busy (eg. in "exclusive" use by another process)
    Busy,
    /// BassWma: the file is protected
    WmaLicense,
    /// BassWma: WM9 is required
    WmaWm9,
    /// BassWma: access denied (user/pass is invalid)
    WmaDenied,
    /// BassWma: no appropriate codec is installed
    WmaCodec,
    /// BassWma: individualization is needed
    WmaIndividual,
    /// BassEnc: ACM codec selection cancelled
    AcmCancel,
    /// BassEnc: Access denied (invalid password)
    CastDenied,
    /// BASSWASAPI: no WASAPI available
    Wasapi,
    /// BASS_AAC: non-streamable due to MP4 atom order ('mdat' before 'moov')
    Mp4Nostream,

    /// Some other mystery error
    Unknown(i32),
}

impl std::fmt::Display for BassError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            BassError::Ok => write!(f, "All is OK"),
            BassError::Mem => write!(f, "Memory error"),
            BassError::FileOpen => write!(f, "Can't open the file or stream"),
            BassError::Driver => write!(f, "Can't find a free or valid driver"),
            BassError::BufLost => write!(f, "The sample buffer was lost"),
            BassError::Handle => write!(f, "Invalid handle"),
            BassError::Format => write!(f, "Unsupported sample format"),
            BassError::Position => write!(f, "Invalid playback position"),
            BassError::Init => write!(f, "General BASS library initialization failed"),
            BassError::Start => write!(f, "BASS was fail to start"),
            BassError::Nocd => write!(f, "No CD in drive"),
            BassError::Cdtrack => write!(f, "Inavlid CD track number"),
            BassError::Already => write!(f, "Already initialized/paused/whatever "),
            BassError::Nopause => write!(f, "Not paused"),
            BassError::Notaudio => write!(f, "Not an audio track"),
            BassError::Nochan => write!(f, "Can't get a free channel"),
            BassError::Illtype => write!(f, "An illegal type was specified"),
            BassError::Illparam => write!(f, "An illegal parameter was specified"),
            BassError::No3d => write!(f, "No 3D support"),
            BassError::Noeax => write!(f, "No EAX suppoer"),
            BassError::Device => write!(f, "Illegal device number"),
            BassError::Noplay => write!(f, "Not playing"),
            BassError::Freq => write!(f, "Illegal sample rate"),
            BassError::Notfile => write!(f, "The stream is not a file stream"),
            BassError::Nohw => write!(f, "No hardware voices available"),
            BassError::Empty => write!(f, "The MOD music has no sequence data"),
            BassError::Nonet => write!(f, "No internet connection could be opened"),
            BassError::Create => write!(f, "Couldn't create the file"),
            BassError::Nofx => write!(f, "Effects are not available"),
            BassError::Playing => write!(f, "The channel is playing"),
            BassError::Notavail => write!(f, "Requested data is not available"),
            BassError::Decode => write!(f, "The channel is a 'decoding channel'"),
            BassError::Dx => write!(f, "A sufficient DirectX version is not installed"),
            BassError::Timeout => write!(f, "Connection timedout"),
            BassError::Fileform => write!(f, "Unsupported file format"),
            BassError::Speaker => write!(f, "Unavailable speaker"),
            BassError::Version => write!(f, "Invalid BASS version"),
            BassError::Codec => write!(f, "Codec is not available or supported"),
            BassError::Ended => write!(f, "The channel or file has ended"),
            BassError::Busy => write!(f, "The device is busy (eg. in \"exclusive\" use by another process)"),
            BassError::WmaLicense => write!(f, "BassWma: the file is protected"),
            BassError::WmaWm9 => write!(f, "BassWma: WM9 is required"),
            BassError::WmaDenied => write!(f, "BassWma: access denied (user or password is invalid)"),
            BassError::WmaCodec => write!(f, "BassWma: no appropriate codec is installed "),
            BassError::WmaIndividual => write!(f, "BassWma: individualization is needed"),
            BassError::AcmCancel => write!(f, "BassEnc: ACM codec selection cancelled"),
            BassError::CastDenied => write!(f, "BassEnc: Access denied (invalid password)"),
            BassError::Wasapi => write!(f, "BASSWASAPI: no WASAPI available"),
            BassError::Mp4Nostream => write!(f, "BASS_AAC: non-streamable due to MP4 atom order ('mdat' before 'moov')"),
            BassError::Unknown(err) => write!(f, "Unknown error code: {}", err),
        }
    }
}


impl BassError {
    /// get the corresponding error for an error code
    pub fn from_code(bass_err:i32) -> Self {
        if let Some(&err) = ERROR_MAP.get(&bass_err) {
            err
        } else {
            Self::Unknown(bass_err)
        }
    }

    /// Get the last error that occurred within bass
    pub fn get_last_error() -> Self {
        Self::from_code(bass_sys::BASS_ErrorGetCode())
    }
}
