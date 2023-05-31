use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DeviceFlags {
    Enabled,
    Default,
    DefaultCom,
    Init,
    Loopback,
}
crate::__impl_BassFlags!(DeviceFlags, [
    (BASS_DEVICE_ENABLED, DeviceFlags::Enabled),
    (BASS_DEVICE_DEFAULT, DeviceFlags::Default),
    // (BASS_DEVICE_DEFAULTCOM, DeviceFlags::DefaultCom),
    (BASS_DEVICE_INIT, DeviceFlags::Init),
    (BASS_DEVICE_LOOPBACK, DeviceFlags::Loopback)
]);


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DeviceType {
    Digital,
    DisplayPort,
    Handset,
    Hdmi,
    Headphones,
    Headset,
    Line,
    Microphone,
    Network,
    Spdif,
    Speakers
}
crate::__impl_BassFlags!(DeviceType, [
    (BASS_DEVICE_TYPE_DIGITAL, DeviceType::Digital),
    // (BASS_DEVICE_DEFAULT, DeviceFlags::Default),
    // // (BASS_DEVICE_DEFAULTCOM, DeviceFlags::DefaultCom),
    // (BASS_DEVICE_INIT, DeviceFlags::Init),
    // (BASS_DEVICE_LOOPBACK, DeviceFlags::Loopback)
]);