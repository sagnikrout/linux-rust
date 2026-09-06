//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/aci.h
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


// SPDX-License-Identifier: GPL-2.0

pub const ACI_SET_MUTE: c_uint = 0x0d;
pub const ACI_SET_POWERAMP: c_uint = 0x0f;
pub const ACI_SET_TUNERMUTE: c_uint = 0xa3;
pub const ACI_SET_TUNERMONO: c_uint = 0xa4;
pub const ACI_SET_IDE: c_uint = 0xd0;
pub const ACI_SET_WSS: c_uint = 0xd1;
pub const ACI_SET_SOLOMODE: c_uint = 0xd2;
pub const ACI_SET_PREAMP: c_uint = 0x03;
pub const ACI_GET_PREAMP: c_uint = 0x21;
pub const ACI_WRITE_TUNE: c_uint = 0xa7;
pub const ACI_READ_TUNERSTEREO: c_uint = 0xa8;
pub const ACI_READ_TUNERSTATION: c_uint = 0xa9;
pub const ACI_READ_VERSION: c_uint = 0xf1;
pub const ACI_READ_IDCODE: c_uint = 0xf2;
pub const ACI_INIT: c_uint = 0xff;
pub const ACI_STATUS: c_uint = 0xf0;
pub const ACI_S_GENERAL: c_uint = 0x00;
pub const ACI_ERROR_OP: c_uint = 0xdf;
// ACI Mixer
// These are the values for the right channel GET registers.
pub const ACI_GET_MASTER: c_uint = 0x03;
pub const ACI_GET_MIC: c_uint = 0x05;
pub const ACI_GET_LINE: c_uint = 0x07;
pub const ACI_GET_CD: c_uint = 0x09;
pub const ACI_GET_SYNTH: c_uint = 0x0b;
pub const ACI_GET_PCM: c_uint = 0x0d;
pub const ACI_GET_LINE1: c_uint = 0x10	/* Radio on PCM20 */;
pub const ACI_GET_LINE2: c_uint = 0x12;
pub const ACI_GET_EQ1: c_uint = 0x22	/* from Bass ... */;
pub const ACI_GET_EQ2: c_uint = 0x24;
pub const ACI_GET_EQ3: c_uint = 0x26;
pub const ACI_GET_EQ4: c_uint = 0x28;
pub const ACI_GET_EQ5: c_uint = 0x2a;
pub const ACI_GET_EQ6: c_uint = 0x2c;
pub const ACI_GET_EQ7: c_uint = 0x2e	/* ... to Treble */;
// And these are the values for the right channel SET registers.
pub const ACI_SET_MASTER: c_uint = 0x00;
pub const ACI_SET_MIC: c_uint = 0x30;
pub const ACI_SET_LINE: c_uint = 0x31;
pub const ACI_SET_CD: c_uint = 0x34;
pub const ACI_SET_SYNTH: c_uint = 0x33;
pub const ACI_SET_PCM: c_uint = 0x32;
pub const ACI_SET_LINE1: c_uint = 0x35	/* Radio on PCM20 */;
pub const ACI_SET_LINE2: c_uint = 0x36;
pub const ACI_SET_EQ1: c_uint = 0x40	/* from Bass ... */;
pub const ACI_SET_EQ2: c_uint = 0x41;
pub const ACI_SET_EQ3: c_uint = 0x42;
pub const ACI_SET_EQ4: c_uint = 0x43;
pub const ACI_SET_EQ5: c_uint = 0x44;
pub const ACI_SET_EQ6: c_uint = 0x45;
pub const ACI_SET_EQ7: c_uint = 0x46	/* ... to Treble */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_miro_aci {
    pub card: *mut snd_card,
    pub aci_port: c_ulong,
    pub aci_vendor: c_int,
    pub aci_product: c_int,
    pub aci_version: c_int,
    pub aci_amp: c_int,
    pub aci_preamp: c_int,
    pub aci_solomode: c_int,
    pub aci_mutex: mutex,
}

extern "C" {
    pub fn snd_aci_cmd(aci: *mut snd_miro_aci, write1: c_int, write2: c_int, write3: c_int) -> c_int;
}
