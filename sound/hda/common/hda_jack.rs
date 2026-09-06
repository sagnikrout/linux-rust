//! Automatically rewritten from C Header to Rust Module
//! Source: sound/hda/common/hda_jack.h
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
// Jack-detection handling for HD-audio
//
// Copyright (c) 2011 Takashi Iwai <tiwai@suse.de>
//

extern "C" {
    pub fn void(: *mut *mut hda_jack_callback_fn) (struct hda_codec, : *mut hda_jack_callback) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hda_jack_callback {
    pub nid: hda_nid_t,
    pub dev_id: c_int,
    pub func: hda_jack_callback_fn,
    pub /: *mut *mut unsigned int private_data; / arbitrary data,
    pub /: *mut *mut unsigned int unsol_res; / unsolicited event bits,
    pub /: *mut *mut *mut hda_jack_tbl jack; / associated jack entry,
    pub next: *mut hda_jack_callback,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hda_jack_tbl {
    pub nid: hda_nid_t,
    pub dev_id: c_int,
    pub /: *mut *mut unsigned char tag; / unsol event tag,
    pub callback: *mut hda_jack_callback,
// jack-detection stuff
    pub /: *mut *mut unsigned int pin_sense; / cached pin-sense value,
    pub /: *mut *mut unsigned int jack_detect:1; / capable of jack-detection?,
    pub /: *mut *mut unsigned int jack_dirty:1; / needs to update?,
    pub /: *mut *mut unsigned int phantom_jack:1; / a fixed, always present port?,
    pub /: *mut *mut unsigned int block_report:1; / in a transitional state - do not report to userspace,
    pub /: *mut *mut hda_nid_t gating_jack; / valid when gating jack plugged,
    pub /: *mut *mut hda_nid_t gated_jack; / gated is dependent on this jack,
    pub /: *mut *mut hda_nid_t key_report_jack; / key reports to this jack,
    pub type: c_int,
    pub button_state: c_int,
    pub jack: *mut snd_jack,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hda_jack_keymap {
    pub type: snd_jack_types,
    pub key: c_int,
}

//
// snd_hda_jack_tbl_get - query the jack-table entry for the given NID
// @codec: the HDA codec
// @nid: pin NID to refer to
//
extern "C" {
    pub fn snd_hda_jack_tbl_get_mst(_arg: codec, _arg: nid, _arg: 0) -> return;
}
extern "C" {
    pub fn snd_hda_jack_tbl_disconnect(codec: *mut hda_codec);
}
extern "C" {
    pub fn snd_hda_jack_tbl_clear(codec: *mut hda_codec);
}
extern "C" {
    pub fn snd_hda_jack_set_dirty_all(codec: *mut hda_codec);
}
//
// snd_hda_jack_detect_enable_callback - enable the jack-detection
// @codec: the HDA codec
// @nid: pin NID to enable
// @cb: callback function to register
//
// In the case of error, the return value will be a pointer embedded with
// errno.  Check and handle the return value appropriately with standard
// macros such as @IS_ERR() and @PTR_ERR().
//
extern "C" {
    pub fn snd_hda_jack_detect_enable_callback_mst(_arg: codec, _arg: nid, _arg: 0, _arg: cb) -> return;
}
extern "C" {
    pub fn snd_hda_jack_pin_sense(codec: *mut hda_codec, nid: hda_nid_t, dev_id: c_int) -> u32;
}
// the jack state returned from snd_hda_jack_detect_state()
//
// snd_hda_jack_detect_state - query pin Presence Detect status
// @codec: the CODEC to sense
// @nid: the pin NID to sense
//
// Query and return the pin's Presence Detect status, as either
// HDA_JACK_NOT_PRESENT, HDA_JACK_PRESENT or HDA_JACK_PHANTOM.
//
extern "C" {
    pub fn snd_hda_jack_detect_state_mst(_arg: codec, _arg: nid, _arg: 0) -> return;
}
//
// snd_hda_jack_detect_mst - Detect the jack
// @codec: the HDA codec
// @nid: pin NID to check jack detection
// @dev_id: pin device entry id
//
// snd_hda_jack_detect - Detect the jack
// @codec: the HDA codec
// @nid: pin NID to check jack detection
//
extern "C" {
    pub fn snd_hda_jack_detect_mst(_arg: codec, _arg: nid, _arg: 0) -> return;
}
extern "C" {
    pub fn is_jack_detectable(codec: *mut hda_codec, nid: hda_nid_t) -> bool;
}
//
// snd_hda_jack_add_kctl - Add a kctl for the given pin
// @codec: the HDA codec
// @nid: pin NID to assign
// @name: string name for the jack
// @phantom_jack: flag to deal as a phantom jack
// @type: jack type bits to be reported, 0 for guessing from pincfg
// @keymap: optional jack / key mapping
//
// This assigns a jack-detection kctl to the given pin.  The kcontrol
// will have the given name and index.
//
extern "C" {
    pub fn snd_hda_jack_report_sync(codec: *mut hda_codec);
}
extern "C" {
    pub fn snd_hda_jack_unsol_event(codec: *mut hda_codec, res: c_uint);
}
extern "C" {
    pub fn snd_hda_jack_poll_all(codec: *mut hda_codec);
}
