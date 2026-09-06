//! Automatically rewritten from C Header to Rust Module
//! Source: sound/usb/mixer_us16x08.h
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
pub const SND_US16X08_MAX_CHANNELS: c_int = 16;
// define some bias, cause some alsa-mixers wont work with
// negative ranges or if mixer-min != 0
//
pub const SND_US16X08_NO_BIAS: c_int = 0;
pub const SND_US16X08_FADER_BIAS: c_int = 127;
pub const SND_US16X08_EQ_HIGHFREQ_BIAS: c_uint = 0x20;
pub const SND_US16X08_COMP_THRESHOLD_BIAS: c_uint = 0x20;
pub const SND_US16X08_COMP_ATTACK_BIAS: c_int = 2;
pub const SND_US16X08_COMP_RELEASE_BIAS: c_int = 1;
// get macro for components of kcontrol private_value

// set macro for kcontrol private_value

// the URB request/type to control Tascam mixers
pub const SND_US16X08_URB_REQUEST: c_uint = 0x1D;
pub const SND_US16X08_URB_REQUESTTYPE: c_uint = 0x40;
// the URB params to retrieve meter ranges
pub const SND_US16X08_URB_METER_REQUEST: c_uint = 0x1e;
pub const SND_US16X08_URB_METER_REQUESTTYPE: c_uint = 0xc0;

// Common Channel control IDs
pub const SND_US16X08_ID_BYPASS: c_uint = 0x45;
pub const SND_US16X08_ID_BUSS_OUT: c_uint = 0x44;
pub const SND_US16X08_ID_PHASE: c_uint = 0x85;
pub const SND_US16X08_ID_MUTE: c_uint = 0x83;
pub const SND_US16X08_ID_FADER: c_uint = 0x81;
pub const SND_US16X08_ID_PAN: c_uint = 0x82;
pub const SND_US16X08_ID_METER: c_uint = 0xB1;
pub const SND_US16X08_ID_EQ_BAND_COUNT: c_int = 4;
pub const SND_US16X08_ID_EQ_PARAM_COUNT: c_int = 4;
// EQ level IDs
pub const SND_US16X08_ID_EQLOWLEVEL: c_uint = 0x01;
pub const SND_US16X08_ID_EQLOWMIDLEVEL: c_uint = 0x02;
pub const SND_US16X08_ID_EQHIGHMIDLEVEL: c_uint = 0x03;
pub const SND_US16X08_ID_EQHIGHLEVEL: c_uint = 0x04;
// EQ frequence IDs
pub const SND_US16X08_ID_EQLOWFREQ: c_uint = 0x11;
pub const SND_US16X08_ID_EQLOWMIDFREQ: c_uint = 0x12;
pub const SND_US16X08_ID_EQHIGHMIDFREQ: c_uint = 0x13;
pub const SND_US16X08_ID_EQHIGHFREQ: c_uint = 0x14;
// EQ width IDs
pub const SND_US16X08_ID_EQLOWMIDWIDTH: c_uint = 0x22;
pub const SND_US16X08_ID_EQHIGHMIDWIDTH: c_uint = 0x23;
pub const SND_US16X08_ID_EQENABLE: c_uint = 0x30;

pub const SND_US16X08_ID_ROUTE: c_uint = 0x00;
// Compressor Ids
pub const SND_US16X08_ID_COMP_BASE: c_uint = 0x32;

pub const SND_US16X08_ID_COMP_COUNT: c_int = 6;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_us16x08_eq_store {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_us16x08_comp_store {
    pub val: [u8; SND_US16X08_ID_COMP_COUNT][SND_US16X08_MAX_CHANNELS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_us16x08_meter_store {
    pub meter_level: [c_int; SND_US16X08_MAX_CHANNELS],
    pub /: *mut *mut int master_level[2]; / level of meter for master output,
    pub /: *mut *mut int comp_index; / round trip channel selector,
    pub /: *mut *mut int comp_active_index; / channel select from user space mixer,
    pub /: *mut *mut int comp_level[16]; / compressor reduction level,
    pub comp_store: *mut snd_us16x08_comp_store,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_us16x08_control_params {
    pub kcontrol_new: *const snd_kcontrol_new,
    pub control_id: c_int,
    pub type: c_int,
    pub num_channels: c_int,
    pub name: *const c_char,
    pub default_val: c_int,
}

extern "C" {
    pub fn snd_us16x08_controls_create(mixer: *mut usb_mixer_interface) -> c_int;
}
