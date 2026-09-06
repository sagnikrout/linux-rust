//! Automatically rewritten from C Header to Rust Module
//! Source: sound/hda/common/hda_controller.h
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
// Common functionality for the alsa driver code base for HD Audio.
//

pub const AZX_DEFAULT_CODECS: c_int = 4;
// driver quirks (capabilities)
// bits 0-7 are used for indicating driver type

// 14 unused

// 19 unused

// 22 unused

// 24 unused

#[repr(C)]
#[derive(Copy, Clone)]
pub struct azx_dev {
    pub core: hdac_stream,
//
// For VIA:
// A flag to ensure DMA position is 0
// when link position is not greater than FIFO size
//
    pub insufficient: bool,
}

// Functions to read/write to hda registers.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hda_controller_ops {
// Disable msi if supported, PCI only
    pub ): *mut *mut int (disable_msi_reset_irq)(struct azx,
// Check if current position is acceptable
    pub azx_dev): *mut *mut *mut int (position_check)(struct azx chip, struct azx_dev,
// enable/disable the link power
    pub enable): *mut *mut *mut int (link_power)(struct azx chip, bool,
// additional hook for PCM
    pub azx_dev): *mut *mut *mut void (pcm_close)(struct azx chip, struct azx_dev,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct azx_pcm {
    pub chip: *mut azx,
    pub pcm: *mut snd_pcm,
    pub codec: *mut hda_codec,
    pub info: *mut hda_pcm,
    pub list: list_head,
}

extern "C" {
    pub fn int(: *mut *mut azx_get_pos_callback_t)(struct azx, : *mut azx_dev) -> typedef unsigned;
}
extern "C" {
    pub fn int(: *mut *mut azx_get_delay_callback_t)(struct azx, : *mut azx_dev, pos: c_uint) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct azx {
    pub bus: hda_bus,
    pub card: *mut snd_card,
    pub pci: *mut pci_dev,
    pub dev_index: c_int,
// chip type specific
    pub driver_type: c_int,
    pub driver_caps: c_uint,
    pub playback_streams: c_int,
    pub playback_index_offset: c_int,
    pub capture_streams: c_int,
    pub capture_index_offset: c_int,
    pub num_streams: c_int,
    pub /: *mut *mut int jackpoll_interval; / jack poll interval in jiffies,
// Register interaction.
    pub ops: *const hda_controller_ops,
// position adjustment callbacks
    pub get_position: [azx_get_pos_callback_t; 2],
    pub get_delay: [azx_get_delay_callback_t; 2],
// locks
    pub /: *mut *mut mutex open_mutex; / Prevents concurrent open/close operations,
// PCM
    pub /: *mut *mut list_head pcm_list; / azx_pcm list,
// HD codec
    pub /: *mut *mut int codec_probe_mask; / copied from probe_mask option,
    pub beep_mode: c_uint,
    pub ctl_dev_id: bool,
// flags
    pub bdl_pos_adj: c_int,
    pub running:1: c_uint,
    pub fallback_to_single_cmd:1: c_uint,
    pub single_cmd:1: c_uint,
    pub msi:1: c_uint,
    pub /: *mut *mut unsigned int probing:1; / codec probing phase,
    pub snoop:1: c_uint,
    pub /: *mut *mut unsigned int uc_buffer:1; / non-cached pages for stream buffers,
    pub align_buffer_size:1: c_uint,
    pub /: *mut *mut unsigned int disabled:1; / disabled by vga_switcheroo,
    pub pm_prepared:1: c_uint,
// GTS present
    pub gts_present:1: c_uint,

    pub saved_azx_dev: azx_dev,

}

//
// macros for easy use
//

// PCM setup
extern "C" {
    pub fn azx_get_position(chip: *mut azx, azx_dev: *mut azx_dev) -> c_uint;
}
extern "C" {
    pub fn azx_get_pos_lpib(chip: *mut azx, azx_dev: *mut azx_dev) -> c_uint;
}
extern "C" {
    pub fn azx_get_pos_posbuf(chip: *mut azx, azx_dev: *mut azx_dev) -> c_uint;
}
// Stream control.
extern "C" {
    pub fn azx_stop_all_streams(chip: *mut azx);
}
// Allocation functions.

// Low level azx interface
extern "C" {
    pub fn azx_init_chip(chip: *mut azx, full_reset: bool);
}
extern "C" {
    pub fn azx_stop_chip(chip: *mut azx);
}

extern "C" {
    pub fn azx_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
// Codec interface
extern "C" {
    pub fn azx_bus_init(chip: *mut azx, model: *const c_char) -> c_int;
}
extern "C" {
    pub fn azx_probe_codecs(chip: *mut azx, max_slots: c_uint) -> c_int;
}
extern "C" {
    pub fn azx_codec_configure(chip: *mut azx) -> c_int;
}
extern "C" {
    pub fn azx_init_streams(chip: *mut azx) -> c_int;
}
extern "C" {
    pub fn azx_add_stream(chip: *mut azx, s: *mut azx_dev, idx: c_int, tag: c_int);
}
extern "C" {
    pub fn azx_free_streams(chip: *mut azx);
}
