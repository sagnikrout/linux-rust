//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/sound/tlv.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note

//
// channel-mapping TLV items
// TLV length must match with num_channels
//
pub const SNDRV_CTL_TLVT_CHMAP_FIXED: c_uint = 0x101	/* fixed channel position */;
pub const SNDRV_CTL_TLVT_CHMAP_VAR: c_uint = 0x102	/* channels freely swappable */;
pub const SNDRV_CTL_TLVT_CHMAP_PAIRED: c_uint = 0x103	/* pair-wise swappable */;
pub const SNDRV_CTL_TLVT_FCP_CHANNEL_LABELS: c_uint = 0x110	/* channel labels */;
//
// TLV structure is right behind the struct snd_ctl_tlv:
// unsigned int type  	- see SNDRV_CTL_TLVT_
// unsigned int length
// .... data aligned to sizeof(unsigned int), use
// block_length = (length + (sizeof(unsigned int) - 1)) &
// ~(sizeof(unsigned int) - 1)) ....
//

// Accessor offsets for TLV data items
pub const SNDRV_CTL_TLVO_TYPE: c_int = 0;
pub const SNDRV_CTL_TLVO_LEN: c_int = 1;

pub const SNDRV_CTL_TLVD_DB_SCALE_MASK: c_uint = 0xffff;
pub const SNDRV_CTL_TLVD_DB_SCALE_MUTE: c_uint = 0x10000;

// Accessor offsets for min, mute and step items in dB scale type TLV
pub const SNDRV_CTL_TLVO_DB_SCALE_MIN: c_int = 2;
pub const SNDRV_CTL_TLVO_DB_SCALE_MUTE_AND_STEP: c_int = 3;
// dB scale specified with min/max values instead of step

// Accessor offsets for min, max items in db-minmax types of TLV.
pub const SNDRV_CTL_TLVO_DB_MINMAX_MIN: c_int = 2;
pub const SNDRV_CTL_TLVO_DB_MINMAX_MAX: c_int = 3;
// linear volume between min_dB and max_dB (.01dB unit)

// Accessor offsets for min, max items in db-linear type of TLV.
pub const SNDRV_CTL_TLVO_DB_LINEAR_MIN: c_int = 2;
pub const SNDRV_CTL_TLVO_DB_LINEAR_MAX: c_int = 3;
// dB range container:
// Items in dB range container must be ordered by their values and by their
// dB values. This implies that larger values must correspond with larger
// dB values (which is also required for all other mixer controls).
//
// Each item is: <min> <max> <TLV>

