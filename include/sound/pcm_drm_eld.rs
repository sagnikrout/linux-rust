//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/pcm_drm_eld.h
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
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum eld_versions {
    ELD_VER_CEA_861D	= 2,
    ELD_VER_PARTIAL		= 31,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cea_audio_coding_types {
    AUDIO_CODING_TYPE_REF_STREAM_HEADER	=  0,
    AUDIO_CODING_TYPE_LPCM			=  1,
    AUDIO_CODING_TYPE_AC3			=  2,
    AUDIO_CODING_TYPE_MPEG1			=  3,
    AUDIO_CODING_TYPE_MP3			=  4,
    AUDIO_CODING_TYPE_MPEG2			=  5,
    AUDIO_CODING_TYPE_AACLC			=  6,
    AUDIO_CODING_TYPE_DTS			=  7,
    AUDIO_CODING_TYPE_ATRAC			=  8,
    AUDIO_CODING_TYPE_SACD			=  9,
    AUDIO_CODING_TYPE_EAC3			= 10,
    AUDIO_CODING_TYPE_DTS_HD		= 11,
    AUDIO_CODING_TYPE_MLP			= 12,
    AUDIO_CODING_TYPE_DST			= 13,
    AUDIO_CODING_TYPE_WMAPRO		= 14,
    AUDIO_CODING_TYPE_REF_CXT		= 15,
// also include valid xtypes below
    AUDIO_CODING_TYPE_HE_AAC		= 15,
    AUDIO_CODING_TYPE_HE_AAC2		= 16,
    AUDIO_CODING_TYPE_MPEG_SURROUND		= 17,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cea_audio_coding_xtypes {
    AUDIO_CODING_XTYPE_HE_REF_CT		= 0,
    AUDIO_CODING_XTYPE_HE_AAC		= 1,
    AUDIO_CODING_XTYPE_HE_AAC2		= 2,
    AUDIO_CODING_XTYPE_MPEG_SURROUND	= 3,
    AUDIO_CODING_XTYPE_FIRST_RESERVED	= 4,
}

//
// CEA Short Audio Descriptor data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_cea_sad {
    pub channels: c_int,
    pub /: *mut *mut int format; / (format == 0) indicates invalid SAD,
    pub rates: c_int,
    pub /: *mut *mut int sample_bits; / for LPCM,
    pub /: *mut *mut int max_bitrate; / for AC3...ATRAC,
    pub /: *mut *mut int profile; / for WMAPRO,
}

pub const ELD_FIXED_BYTES: c_int = 20;
pub const ELD_MAX_SIZE: c_int = 256;
pub const ELD_MAX_MNL: c_int = 16;
pub const ELD_MAX_SAD: c_int = 16;

//
// ELD: EDID Like Data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_parsed_hdmi_eld {
//
// all fields will be cleared before updating ELD
//
    pub baseline_len: c_int,
    pub eld_ver: c_int,
    pub cea_edid_ver: c_int,
    pub 1]: char monitor_name[ELD_MAX_MNL +,
    pub manufacture_id: c_int,
    pub product_id: c_int,
    pub port_id: u64,
    pub support_hdcp: c_int,
    pub support_ai: c_int,
    pub conn_type: c_int,
    pub aud_synch_delay: c_int,
    pub spk_alloc: c_int,
    pub sad_count: c_int,
    pub sad: [snd_cea_sad; ELD_MAX_SAD],
}

extern "C" {
    pub fn snd_pcm_hw_constraint_eld(runtime: *mut snd_pcm_runtime, eld: *mut c_void) -> c_int;
}
extern "C" {
    pub fn snd_show_eld(dev: *mut device, e: *mut snd_parsed_hdmi_eld);
}

