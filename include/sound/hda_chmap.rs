//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/hda_chmap.h
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
//
// For multichannel support
//

pub const SND_PRINT_CHANNEL_ALLOCATION_ADVISED_BUFSIZE: c_int = 80;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdac_cea_channel_speaker_allocation {
    pub ca_index: c_int,
    pub speakers: [c_int; 8],
// derived values, just for convenience
    pub channels: c_int,
    pub spk_mask: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdac_chmap_ops {
//
// Helpers for producing the channel map TLVs. These can be overridden
// for devices that have non-standard mapping requirements.
//
    pub channels): *mut *mut hdac_cea_channel_speaker_allocation cap, int,
    pub channels): *mut *mut unsigned int chmap, int,
// check that the user-given chmap is supported
    pub chmap): *mut int channels, unsigned char,
    pub pcm_idx): *mut *mut *mut int (get_spk_alloc)(struct hdac_device hdac, int,
    pub chmap): *mut c_uchar,
    pub prepared): *mut *mut unsigned char chmap, int,
    pub pcm_idx): *mut *mut *mut bool (is_pcm_attached)(struct hdac_device hdac, int,
// get and set channel assigned to each HDMI ASP (audio sample packet) slot
    pub asp_slot): hda_nid_t pin_nid, int,
    pub channel): hda_nid_t pin_nid, int asp_slot, int,
    pub chs): hda_nid_t cvt_nid, int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdac_chmap {
    pub /: *mut *mut unsigned int channels_max; / max over all cvts,
    pub ops: hdac_chmap_ops,
    pub hdac: *mut hdac_device,
}

extern "C" {
    pub fn snd_hdac_get_active_channels(ca: c_int) -> c_int;
}
extern "C" {
    pub fn snd_hdac_print_channel_allocation(spk_alloc: c_int, buf: *mut c_char, buflen: c_int);
}
extern "C" {
    pub fn snd_hdac_chmap_to_spk_mask(c: c_uchar) -> c_int;
}
extern "C" {
    pub fn snd_hdac_spk_to_chmap(spk: c_int) -> c_int;
}
