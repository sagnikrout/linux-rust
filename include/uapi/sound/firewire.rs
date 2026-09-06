//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/sound/firewire.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

// events can be read() from the hwdep device
pub const SNDRV_FIREWIRE_EVENT_LOCK_STATUS: c_uint = 0x000010cc;
pub const SNDRV_FIREWIRE_EVENT_DICE_NOTIFICATION: c_uint = 0xd1ce004e;
pub const SNDRV_FIREWIRE_EVENT_EFW_RESPONSE: c_uint = 0x4e617475;
pub const SNDRV_FIREWIRE_EVENT_DIGI00X_MESSAGE: c_uint = 0x746e736c;
pub const SNDRV_FIREWIRE_EVENT_MOTU_NOTIFICATION: c_uint = 0x64776479;
pub const SNDRV_FIREWIRE_EVENT_TASCAM_CONTROL: c_uint = 0x7473636d;
pub const SNDRV_FIREWIRE_EVENT_MOTU_REGISTER_DSP_CHANGE: c_uint = 0x4d545244;
pub const SNDRV_FIREWIRE_EVENT_FF400_MESSAGE: c_uint = 0x4f6c6761;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_firewire_event_common {
    pub /: *mut *mut unsigned int type; / SNDRV_FIREWIRE_EVENT_xxx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_firewire_event_lock_status {
    pub type: c_uint,
    pub /: *mut *mut unsigned int status; / 0/1 = unlocked/locked,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_firewire_event_dice_notification {
    pub type: c_uint,
    pub /: *mut *mut unsigned int notification; / DICE-specific bits,
}

// each field should be in big endian
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_efw_transaction {
    pub length: __be32,
    pub version: __be32,
    pub seqnum: __be32,
    pub category: __be32,
    pub command: __be32,
    pub status: __be32,
    pub params: [__be32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_firewire_event_efw_response {
    pub type: c_uint,
    pub /: *mut *mut __be32 response[]; / some responses,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_firewire_event_digi00x_message {
    pub type: c_uint,
    pub /: *mut *mut __u32 message; / Digi00x-specific message,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_firewire_event_motu_notification {
    pub type: c_uint,
    pub /: *mut *mut __u32 message; / MOTU-specific bits.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_firewire_tascam_change {
    pub index: c_uint,
    pub before: __be32,
    pub after: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_firewire_event_tascam_control {
    pub type: c_uint,
    pub changes: [snd_firewire_tascam_change; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_firewire_event_motu_register_dsp_change {
    pub type: c_uint,
    pub /: *mut *mut __u32 count; / The number of changes.,
    pub /: *mut *mut __u32 changes[]; / Encoded event for change of register DSP.,
}

//
// struct snd_firewire_event_ff400_message - the container for message from Fireface 400 when
// operating hardware knob.
//
// @type: Fixed to SNDRV_FIREWIRE_EVENT_FF400_MESSAGE.
// @message_count: The number of messages.
// @messages: Array of @message_count messages
// @messages.message: The messages expressing hardware knob operation.
// @messages.tstamp: The isochronous cycle at which the request subaction of asynchronous
// transaction was sent to deliver the message. It has 16-bit unsigned integer
// value. The higher 3 bits of value expresses the lower three bits of second
// field in the format of CYCLE_TIME, up to 7. The remaining 13 bits express cycle
// field up to 7999.
//
// The structure expresses message transmitted by Fireface 400 when operating hardware knob.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_firewire_event_ff400_message {
    pub type: c_uint,
    pub message_count: c_uint,
    pub message: __u32,
    pub tstamp: __u32,
    pub messages: [}; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union snd_firewire_event {
    pub common: snd_firewire_event_common,
    pub lock_status: snd_firewire_event_lock_status,
    pub dice_notification: snd_firewire_event_dice_notification,
    pub efw_response: snd_firewire_event_efw_response,
    pub digi00x_message: snd_firewire_event_digi00x_message,
    pub tascam_control: snd_firewire_event_tascam_control,
    pub motu_notification: snd_firewire_event_motu_notification,
    pub motu_register_dsp_change: snd_firewire_event_motu_register_dsp_change,
    pub ff400_message: snd_firewire_event_ff400_message,
}

pub const SNDRV_FIREWIRE_TYPE_DICE: c_int = 1;
pub const SNDRV_FIREWIRE_TYPE_FIREWORKS: c_int = 2;
pub const SNDRV_FIREWIRE_TYPE_BEBOB: c_int = 3;
pub const SNDRV_FIREWIRE_TYPE_OXFW: c_int = 4;
pub const SNDRV_FIREWIRE_TYPE_DIGI00X: c_int = 5;
pub const SNDRV_FIREWIRE_TYPE_TASCAM: c_int = 6;
pub const SNDRV_FIREWIRE_TYPE_MOTU: c_int = 7;
pub const SNDRV_FIREWIRE_TYPE_FIREFACE: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_firewire_get_info {
    pub /: *mut *mut unsigned int type; / SNDRV_FIREWIRE_TYPE_xxx,
    pub /: *mut *mut unsigned int card; / same as fw_cdev_get_info.card,
    pub guid: [c_uchar; 8],
    pub /: *mut *mut char device_name[16]; / device node in /dev,
}

//
// SNDRV_FIREWIRE_IOCTL_LOCK prevents the driver from streaming.
// Returns -EBUSY if the driver is already streaming.
//
pub const SNDRV_FIREWIRE_TASCAM_STATE_COUNT: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_firewire_tascam_state {
    pub data: [__be32; SNDRV_FIREWIRE_TASCAM_STATE_COUNT],
}

//
// In below MOTU models, software is allowed to control their DSP by accessing to registers.
// - 828mk2
// - 896hd
// - Traveler
// - 8 pre
// - Ultralite
// - 4 pre
// - Audio Express
//
// On the other hand, the status of DSP is split into specific messages included in the sequence of
// isochronous packet. ALSA firewire-motu driver gathers the messages and allow userspace applications
// to read it via ioctl. In 828mk2, 896hd, and Traveler, hardware meter for all of physical inputs
// are put into the message, while one pair of physical outputs is selected. The selection is done by
// LSB one byte in asynchronous write quadlet transaction to 0x'ffff'f000'0b2c.
//
// I note that V3HD/V4HD uses asynchronous transaction for the purpose. The destination address is
// registered to 0x'ffff'f000'0b38 and '0b3c by asynchronous write quadlet request. The size of
// message differs between 23 and 51 quadlets. For the case, the number of mixer bus can be extended
// up to 12.
//
pub const SNDRV_FIREWIRE_MOTU_REGISTER_DSP_METER_INPUT_COUNT: c_int = 24;
pub const SNDRV_FIREWIRE_MOTU_REGISTER_DSP_METER_OUTPUT_COUNT: c_int = 24;

//
// struct snd_firewire_motu_register_dsp_meter - the container for meter information in DSP
// controlled by register access
// @data: Signal level meters. The mapping between position and input/output channel is
// model-dependent.
//
// The structure expresses the part of DSP status for hardware meter. The u8 storage includes linear
// value for audio signal level between 0x00 and 0x7f.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_firewire_motu_register_dsp_meter {
    pub data: [__u8; SNDRV_FIREWIRE_MOTU_REGISTER_DSP_METER_COUNT],
}

pub const SNDRV_FIREWIRE_MOTU_REGISTER_DSP_MIXER_COUNT: c_int = 4;
pub const SNDRV_FIREWIRE_MOTU_REGISTER_DSP_MIXER_SRC_COUNT: c_int = 20;
pub const SNDRV_FIREWIRE_MOTU_REGISTER_DSP_INPUT_COUNT: c_int = 10;

//
// struct snd_firewire_motu_register_dsp_parameter - the container for parameters
// of DSP controlled by register access.
// @mixer: aggregate of @mixer.source and @mixer.output
// @mixer.source.gain: The gain of source to mixer.
// @mixer.source.pan: The L/R balance of source to mixer.
// @mixer.source.flag: The flag of source to mixer, including mute, solo.
// @mixer.source.paired_balance: The L/R balance of paired source to mixer, only for 4 pre and
// Audio Express.
// @mixer.source.paired_width: The width of paired source to mixer, only for 4 pre and
// Audio Express.
// @mixer.output: FIXME
// @mixer.output.paired_volume: The volume of paired output from mixer.
// @mixer.output.paired_flag: The flag of paired output from mixer.
// @output: output parameters
// @output.main_paired_volume: The volume of paired main output.
// @output.hp_paired_volume: The volume of paired hp output.
// @output.hp_paired_assignment: The source assigned to paired hp output.
// @output.reserved: Padding for 32-bit alignment for future extension.
// @line_input: line input parameters
// @line_input.boost_flag: The flags of boost for line inputs, only for 828mk2 and Traveler.
// @line_input.nominal_level_flag: The flags of nominal level for line inputs, only for 828mk2 and
// Traveler.
// @line_input.reserved: Padding for 32-bit alignment for future extension.
// @input: input parameters
// @input.gain_and_invert: The value including gain and invert for input, only for Ultralite, 4 pre
// and Audio Express.
// @input.flag: The flag of input; e.g. jack detection, phantom power, and pad, only for Ultralite,
// 4 pre and Audio express.
// @reserved: Padding so that the size of structure is kept to 512 bytes, but for future extension.
//
// The structure expresses the set of parameters for DSP controlled by register access.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_firewire_motu_register_dsp_parameter {
    pub gain: [__u8; SNDRV_FIREWIRE_MOTU_REGISTER_DSP_MIXER_SRC_COUNT],
    pub pan: [__u8; SNDRV_FIREWIRE_MOTU_REGISTER_DSP_MIXER_SRC_COUNT],
    pub flag: [__u8; SNDRV_FIREWIRE_MOTU_REGISTER_DSP_MIXER_SRC_COUNT],
    pub paired_balance: [__u8; SNDRV_FIREWIRE_MOTU_REGISTER_DSP_MIXER_SRC_COUNT],
    pub paired_width: [__u8; SNDRV_FIREWIRE_MOTU_REGISTER_DSP_MIXER_SRC_COUNT],
    pub source: [}; SNDRV_FIREWIRE_MOTU_REGISTER_DSP_MIXER_COUNT],
    pub paired_volume: [__u8; SNDRV_FIREWIRE_MOTU_REGISTER_DSP_MIXER_COUNT],
    pub paired_flag: [__u8; SNDRV_FIREWIRE_MOTU_REGISTER_DSP_MIXER_COUNT],
    pub output: },
    pub mixer: },
    pub main_paired_volume: __u8,
    pub hp_paired_volume: __u8,
    pub hp_paired_assignment: __u8,
    pub reserved: [__u8; 5],
    pub output: },
    pub boost_flag: __u8,
    pub nominal_level_flag: __u8,
    pub reserved: [__u8; 6],
    pub line_input: },
    pub gain_and_invert: [__u8; SNDRV_FIREWIRE_MOTU_REGISTER_DSP_ALIGNED_INPUT_COUNT],
    pub flag: [__u8; SNDRV_FIREWIRE_MOTU_REGISTER_DSP_ALIGNED_INPUT_COUNT],
    pub input: },
    pub reserved: [__u8; 64],
}

//
// In below MOTU models, software is allowed to control their DSP by command in frame of
// asynchronous transaction to 0x'ffff'0001'0000:
//
// - 828 mk3 (FireWire only and Hybrid)
// - 896 mk3 (FireWire only and Hybrid)
// - Ultralite mk3 (FireWire only and Hybrid)
// - Traveler mk3
// - Track 16
//
// On the other hand, the states of hardware meter is split into specific messages included in the
// sequence of isochronous packet. ALSA firewire-motu driver gathers the message and allow userspace
// application to read it via ioctl.
//
pub const SNDRV_FIREWIRE_MOTU_COMMAND_DSP_METER_COUNT: c_int = 400;
//
// struct snd_firewire_motu_command_dsp_meter - the container for meter information in DSP
// controlled by command
// @data: Signal level meters. The mapping between position and signal channel is model-dependent.
//
// The structure expresses the part of DSP status for hardware meter. The 32-bit storage is
// estimated to include IEEE 764 32-bit single precision floating point (binary32) value. It is
// expected to be linear value (not logarithm) for audio signal level between 0.0 and +1.0.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_firewire_motu_command_dsp_meter {
    pub data: [__u32; SNDRV_FIREWIRE_MOTU_COMMAND_DSP_METER_COUNT],    pub data: [float; SNDRV_FIREWIRE_MOTU_COMMAND_DSP_METER_COUNT],
}
