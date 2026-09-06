//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/lirc.h
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
//
// lirc.h - linux infrared remote control header file
//

pub const PULSE_BIT: c_uint = 0x01000000;
pub const PULSE_MASK: c_uint = 0x00FFFFFF;
pub const LIRC_MODE2_SPACE: c_uint = 0x00000000;
pub const LIRC_MODE2_PULSE: c_uint = 0x01000000;
pub const LIRC_MODE2_FREQUENCY: c_uint = 0x02000000;
pub const LIRC_MODE2_TIMEOUT: c_uint = 0x03000000;
pub const LIRC_MODE2_OVERFLOW: c_uint = 0x04000000;
pub const LIRC_VALUE_MASK: c_uint = 0x00FFFFFF;
pub const LIRC_MODE2_MASK: c_uint = 0xFF000000;

// used heavily by lirc userspace

// lirc compatible hardware features

pub const LIRC_MODE_RAW: c_uint = 0x00000001;
pub const LIRC_MODE_PULSE: c_uint = 0x00000002;
pub const LIRC_MODE_MODE2: c_uint = 0x00000004;
pub const LIRC_MODE_SCANCODE: c_uint = 0x00000008;
pub const LIRC_MODE_LIRCCODE: c_uint = 0x00000010;

pub const LIRC_CAN_SEND_MASK: c_uint = 0x0000003f;
pub const LIRC_CAN_SET_SEND_CARRIER: c_uint = 0x00000100;
pub const LIRC_CAN_SET_SEND_DUTY_CYCLE: c_uint = 0x00000200;
pub const LIRC_CAN_SET_TRANSMITTER_MASK: c_uint = 0x00000400;

pub const LIRC_CAN_SET_REC_CARRIER_RANGE: c_uint = 0x80000000;
pub const LIRC_CAN_GET_REC_RESOLUTION: c_uint = 0x20000000;
pub const LIRC_CAN_SET_REC_TIMEOUT: c_uint = 0x10000000;
pub const LIRC_CAN_MEASURE_CARRIER: c_uint = 0x02000000;
pub const LIRC_CAN_USE_WIDEBAND_RECEIVER: c_uint = 0x04000000;

//
// Unused features. These features were never implemented, in tree or
// out of tree. These definitions are here so not to break the lircd build.
//
pub const LIRC_CAN_SET_REC_FILTER: c_int = 0;
pub const LIRC_CAN_NOTIFY_DECODE: c_int = 0;
// IOCTL commands for lirc driver

// code length in bits, currently only for LIRC_MODE_LIRCCODE

// Note: these can reset the according pulse_width

//
// when a timeout != 0 is set the driver will send a
// LIRC_MODE2_TIMEOUT data packet, otherwise LIRC_MODE2_TIMEOUT is
// never sent, timeout is disabled by default
//

// 1 enables, 0 disables timeout reports in MODE2

//
// if enabled from the next key press on the driver will send
// LIRC_MODE2_FREQUENCY packets
//

//
// to set a range use LIRC_SET_REC_CARRIER_RANGE with the
// lower bound first and later LIRC_SET_REC_CARRIER with the upper bound
//

//
// Return the recording timeout, which is either set by
// the ioctl LIRC_SET_REC_TIMEOUT or by the kernel after setting the protocols.
//

//
// struct lirc_scancode - decoded scancode with protocol for use with
// LIRC_MODE_SCANCODE
//
// @timestamp: Timestamp in nanoseconds using CLOCK_MONOTONIC when IR
// was decoded.
// @flags: should be 0 for transmit. When receiving scancodes,
// LIRC_SCANCODE_FLAG_TOGGLE or LIRC_SCANCODE_FLAG_REPEAT can be set
// depending on the protocol
// @rc_proto: see enum rc_proto
// @keycode: the translated keycode. Set to 0 for transmit.
// @scancode: the scancode received or to be sent
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lirc_scancode {
    pub timestamp: __u64,
    pub flags: __u16,
    pub rc_proto: __u16,
    pub keycode: __u32,
    pub scancode: __u64,
}

// Set if the toggle bit of rc-5 or rc-6 is enabled
pub const LIRC_SCANCODE_FLAG_TOGGLE: c_int = 1;
// Set if this is a nec or sanyo repeat
pub const LIRC_SCANCODE_FLAG_REPEAT: c_int = 2;
//
// enum rc_proto - the Remote Controller protocol
//
// @RC_PROTO_UNKNOWN: Protocol not known
// @RC_PROTO_OTHER: Protocol known but proprietary
// @RC_PROTO_RC5: Philips RC5 protocol
// @RC_PROTO_RC5X_20: Philips RC5x 20 bit protocol
// @RC_PROTO_RC5_SZ: StreamZap variant of RC5
// @RC_PROTO_JVC: JVC protocol
// @RC_PROTO_SONY12: Sony 12 bit protocol
// @RC_PROTO_SONY15: Sony 15 bit protocol
// @RC_PROTO_SONY20: Sony 20 bit protocol
// @RC_PROTO_NEC: NEC protocol
// @RC_PROTO_NECX: Extended NEC protocol
// @RC_PROTO_NEC32: NEC 32 bit protocol
// @RC_PROTO_SANYO: Sanyo protocol
// @RC_PROTO_MCIR2_KBD: RC6-ish MCE keyboard
// @RC_PROTO_MCIR2_MSE: RC6-ish MCE mouse
// @RC_PROTO_RC6_0: Philips RC6-0-16 protocol
// @RC_PROTO_RC6_6A_20: Philips RC6-6A-20 protocol
// @RC_PROTO_RC6_6A_24: Philips RC6-6A-24 protocol
// @RC_PROTO_RC6_6A_32: Philips RC6-6A-32 protocol
// @RC_PROTO_RC6_MCE: MCE (Philips RC6-6A-32 subtype) protocol
// @RC_PROTO_SHARP: Sharp protocol
// @RC_PROTO_XMP: XMP protocol
// @RC_PROTO_CEC: CEC protocol
// @RC_PROTO_IMON: iMon Pad protocol
// @RC_PROTO_RCMM12: RC-MM protocol 12 bits
// @RC_PROTO_RCMM24: RC-MM protocol 24 bits
// @RC_PROTO_RCMM32: RC-MM protocol 32 bits
// @RC_PROTO_XBOX_DVD: Xbox DVD Movie Playback Kit protocol
// @RC_PROTO_MAX: Maximum value of enum rc_proto
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rc_proto {
    RC_PROTO_UNKNOWN	= 0,
    RC_PROTO_OTHER		= 1,
    RC_PROTO_RC5		= 2,
    RC_PROTO_RC5X_20	= 3,
    RC_PROTO_RC5_SZ		= 4,
    RC_PROTO_JVC		= 5,
    RC_PROTO_SONY12		= 6,
    RC_PROTO_SONY15		= 7,
    RC_PROTO_SONY20		= 8,
    RC_PROTO_NEC		= 9,
    RC_PROTO_NECX		= 10,
    RC_PROTO_NEC32		= 11,
    RC_PROTO_SANYO		= 12,
    RC_PROTO_MCIR2_KBD	= 13,
    RC_PROTO_MCIR2_MSE	= 14,
    RC_PROTO_RC6_0		= 15,
    RC_PROTO_RC6_6A_20	= 16,
    RC_PROTO_RC6_6A_24	= 17,
    RC_PROTO_RC6_6A_32	= 18,
    RC_PROTO_RC6_MCE	= 19,
    RC_PROTO_SHARP		= 20,
    RC_PROTO_XMP		= 21,
    RC_PROTO_CEC		= 22,
    RC_PROTO_IMON		= 23,
    RC_PROTO_RCMM12		= 24,
    RC_PROTO_RCMM24		= 25,
    RC_PROTO_RCMM32		= 26,
    RC_PROTO_XBOX_DVD	= 27,
    RC_PROTO_MAX		= RC_PROTO_XBOX_DVD,
}
