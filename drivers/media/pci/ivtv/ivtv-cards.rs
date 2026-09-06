//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/ivtv/ivtv-cards.h
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Supported cards

pub const IVTV_CARD_LAST: c_int = 27;
// Variants of existing cards but with the same PCI IDs. The driver
// PVR-350 V1 (uses saa7114)

// 2 variants of Kuroutoshikou CX23416GYC-STVLP (Yuan MPG600GR OEM)

// system vendor and device IDs
pub const PCI_VENDOR_ID_ICOMP: c_uint = 0x4444;
pub const PCI_DEVICE_ID_IVTV15: c_uint = 0x0803;
pub const PCI_DEVICE_ID_IVTV16: c_uint = 0x0016;
// subsystem vendor ID
pub const IVTV_PCI_ID_HAUPPAUGE: c_uint = 0x0070;
pub const IVTV_PCI_ID_HAUPPAUGE_ALT1: c_uint = 0x0270;
pub const IVTV_PCI_ID_HAUPPAUGE_ALT2: c_uint = 0x4070;
pub const IVTV_PCI_ID_ADAPTEC: c_uint = 0x9005;
pub const IVTV_PCI_ID_ASUSTEK: c_uint = 0x1043;
pub const IVTV_PCI_ID_AVERMEDIA: c_uint = 0x1461;
pub const IVTV_PCI_ID_YUAN1: c_uint = 0x12ab;
pub const IVTV_PCI_ID_YUAN2: c_uint = 0xff01;
pub const IVTV_PCI_ID_YUAN3: c_uint = 0xffab;
pub const IVTV_PCI_ID_YUAN4: c_uint = 0xfbab;
pub const IVTV_PCI_ID_DIAMONDMM: c_uint = 0xff92;
pub const IVTV_PCI_ID_IODATA: c_uint = 0x10fc;
pub const IVTV_PCI_ID_MELCO: c_uint = 0x1154;
pub const IVTV_PCI_ID_GOTVIEW1: c_uint = 0xffac;
pub const IVTV_PCI_ID_GOTVIEW2: c_uint = 0xffad;
pub const IVTV_PCI_ID_SONY: c_uint = 0x104d;
// hardware flags, no gaps allowed
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ivtv_hw_bits {
    IVTV_HW_BIT_CX25840,
    IVTV_HW_BIT_SAA7115,
    IVTV_HW_BIT_SAA7127,
    IVTV_HW_BIT_MSP34XX,
    IVTV_HW_BIT_TUNER,
    IVTV_HW_BIT_WM8775,
    IVTV_HW_BIT_CS53L32A,
    IVTV_HW_BIT_TVEEPROM,
    IVTV_HW_BIT_SAA7114,
    IVTV_HW_BIT_UPD64031A,
    IVTV_HW_BIT_UPD6408X,
    IVTV_HW_BIT_SAA717X,
    IVTV_HW_BIT_WM8739,
    IVTV_HW_BIT_VP27SMPX,
    IVTV_HW_BIT_M52790,
    IVTV_HW_BIT_GPIO,
    IVTV_HW_BIT_I2C_IR_RX_AVER,
    IVTV_HW_BIT_I2C_IR_RX_HAUP_EXT,		 /* External before internal */
    IVTV_HW_BIT_I2C_IR_RX_HAUP_INT,
    IVTV_HW_BIT_Z8F0811_IR_HAUP,
    IVTV_HW_BIT_I2C_IR_RX_ADAPTEC,

    IVTV_HW_MAX_BITS	/* Should be the last one */
}

// video inputs
pub const IVTV_CARD_INPUT_VID_TUNER: c_int = 1;
pub const IVTV_CARD_INPUT_SVIDEO1: c_int = 2;
pub const IVTV_CARD_INPUT_SVIDEO2: c_int = 3;
pub const IVTV_CARD_INPUT_COMPOSITE1: c_int = 4;
pub const IVTV_CARD_INPUT_COMPOSITE2: c_int = 5;
pub const IVTV_CARD_INPUT_COMPOSITE3: c_int = 6;
// audio inputs
pub const IVTV_CARD_INPUT_AUD_TUNER: c_int = 1;
pub const IVTV_CARD_INPUT_LINE_IN1: c_int = 2;
pub const IVTV_CARD_INPUT_LINE_IN2: c_int = 3;
pub const IVTV_CARD_MAX_VIDEO_INPUTS: c_int = 6;
pub const IVTV_CARD_MAX_AUDIO_INPUTS: c_int = 3;
pub const IVTV_CARD_MAX_TUNERS: c_int = 3;
// SAA71XX HW inputs
pub const IVTV_SAA71XX_COMPOSITE0: c_int = 0;
pub const IVTV_SAA71XX_COMPOSITE1: c_int = 1;
pub const IVTV_SAA71XX_COMPOSITE2: c_int = 2;
pub const IVTV_SAA71XX_COMPOSITE3: c_int = 3;
pub const IVTV_SAA71XX_COMPOSITE4: c_int = 4;
pub const IVTV_SAA71XX_COMPOSITE5: c_int = 5;
pub const IVTV_SAA71XX_SVIDEO0: c_int = 6;
pub const IVTV_SAA71XX_SVIDEO1: c_int = 7;
pub const IVTV_SAA71XX_SVIDEO2: c_int = 8;
pub const IVTV_SAA71XX_SVIDEO3: c_int = 9;
// SAA717X needs to mark the tuner input by ORing with this flag
pub const IVTV_SAA717X_TUNER_FLAG: c_uint = 0x80;
// Dummy HW input
pub const IVTV_DUMMY_AUDIO: c_int = 0;
// GPIO HW inputs
pub const IVTV_GPIO_TUNER: c_int = 0;
pub const IVTV_GPIO_LINE_IN: c_int = 1;
// SAA717X HW inputs
pub const IVTV_SAA717X_IN0: c_int = 0;
pub const IVTV_SAA717X_IN1: c_int = 1;
pub const IVTV_SAA717X_IN2: c_int = 2;
// V4L2 capability aliases

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ivtv_card_video_input {
    pub /: *mut *mut u8 video_type; / video input type,
    pub /: *mut *mut u8 audio_index; / index in ivtv_card_audio_input array,
    pub /: *mut *mut u16 video_input; / hardware video input,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ivtv_card_audio_input {
    pub /: *mut *mut u8 audio_type; / audio input type,
    pub /: *mut *mut u32 audio_input; / hardware audio input,
    pub a: *mut *mut u16 muxer_input; / hardware muxer input for boards with,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ivtv_card_output {
    pub name: [u8; 32],
    pub /: *mut *mut u16 video_output; / hardware video output,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ivtv_card_pci_info {
    pub device: u16,
    pub subsystem_vendor: u16,
    pub subsystem_device: u16,
}

// GPIO definitions
// The mask is the set of bits used by the operation
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ivtv_gpio_init {
    pub /: *mut *mut u16 direction; / DIR setting. Leave to 0 if no init is needed,
    pub initial_value: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ivtv_gpio_video_input {
    pub /: *mut *mut u16 mask; / leave to 0 if not supported,
    pub tuner: u16,
    pub composite: u16,
    pub svideo: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ivtv_gpio_audio_input {
    pub /: *mut *mut u16 mask; / leave to 0 if not supported,
    pub tuner: u16,
    pub linein: u16,
    pub radio: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ivtv_gpio_audio_mute {
    pub /: *mut *mut u16 mask; / leave to 0 if not supported,
    pub /: *mut *mut u16 mute; / set this value to mute, 0 to unmute,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ivtv_gpio_audio_mode {
    pub /: *mut *mut u16 mask; / leave to 0 if not supported,
    pub /: *mut *mut u16 mono; / set audio to mono,
    pub /: *mut *mut u16 stereo; / set audio to stereo,
    pub /: *mut *mut u16 lang1; / set audio to the first language,
    pub /: *mut *mut u16 lang2; / set audio to the second language,
    pub /: *mut *mut u16 both; / both languages are output,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ivtv_gpio_audio_freq {
    pub /: *mut *mut u16 mask; / leave to 0 if not supported,
    pub f32000: u16,
    pub f44100: u16,
    pub f48000: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ivtv_gpio_audio_detect {
    pub /: *mut *mut u16 mask; / leave to 0 if not supported,
    pub then: *mut *mut u16 stereo; / if the input matches this value,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ivtv_card_tuner {
    pub /: *mut *mut v4l2_std_id std; / standard for which the tuner is suitable,
    pub /: *mut *mut int tuner; / tuner ID (from tuner.h),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ivtv_card_tuner_i2c {
    pub /: *mut *mut unsigned short radio[2];/ radio tuner i2c address to probe,
    pub /: *mut *mut unsigned short demod[2];/ demodulator i2c address to probe,
    pub /: *mut *mut unsigned short tv[4]; / tv tuner i2c addresses to probe,
}

// for card information/parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ivtv_card {
    pub type: c_int,
    pub name: *mut c_char,
    pub comment: *mut c_char,
    pub v4l2_capabilities: u32,
    pub /: *mut *mut u32 hw_video; / hardware used to process video,
    pub /: *mut *mut u32 hw_audio; / hardware used to process audio,
    pub /: *mut *mut u32 hw_audio_ctrl; / hardware used for the V4L2 controls (only 1 dev allowed),
    pub /: *mut *mut u32 hw_muxer; / hardware used to multiplex audio input,
    pub /: *mut *mut u32 hw_all; / all hardware used by the board,
    pub video_inputs: [ivtv_card_video_input; IVTV_CARD_MAX_VIDEO_INPUTS],
    pub audio_inputs: [ivtv_card_audio_input; IVTV_CARD_MAX_AUDIO_INPUTS],
    pub radio_input: ivtv_card_audio_input,
    pub nof_outputs: c_int,
    pub video_outputs: *const ivtv_card_output,
    pub /: *mut *mut u8 gr_config; / config byte for the ghost reduction device,
    pub /: *mut *mut u8 xceive_pin; / XCeive tuner GPIO reset pin,
// GPIO card-specific settings
    pub gpio_init: ivtv_gpio_init,
    pub gpio_video_input: ivtv_gpio_video_input,
    pub gpio_audio_input: ivtv_gpio_audio_input,
    pub gpio_audio_mute: ivtv_gpio_audio_mute,
    pub gpio_audio_mode: ivtv_gpio_audio_mode,
    pub gpio_audio_freq: ivtv_gpio_audio_freq,
    pub gpio_audio_detect: ivtv_gpio_audio_detect,
    pub tuners: [ivtv_card_tuner; IVTV_CARD_MAX_TUNERS],
    pub i2c: *mut ivtv_card_tuner_i2c,
// list of device and subsystem vendor/devices that
    pub pci_list: *const ivtv_card_pci_info,
}

extern "C" {
    pub fn ivtv_get_input(itv: *mut ivtv, index: u16, input: *mut v4l2_input) -> c_int;
}
extern "C" {
    pub fn ivtv_get_output(itv: *mut ivtv, index: u16, output: *mut v4l2_output) -> c_int;
}
extern "C" {
    pub fn ivtv_get_audio_input(itv: *mut ivtv, index: u16, input: *mut v4l2_audio) -> c_int;
}
extern "C" {
    pub fn ivtv_get_audio_output(itv: *mut ivtv, index: u16, output: *mut v4l2_audioout) -> c_int;
}
