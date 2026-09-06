//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/hda_regmap.h
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
// HD-audio regmap helpers
//

pub const AC_AMP_FAKE_MUTE: c_uint = 0x10	/* fake mute bit set to amp verbs */;
extern "C" {
    pub fn snd_hdac_regmap_init(codec: *mut hdac_device) -> c_int;
}
extern "C" {
    pub fn snd_hdac_regmap_exit(codec: *mut hdac_device);
}
extern "C" {
    pub fn snd_hdac_regmap_sync(codec: *mut hdac_device);
}
//
// snd_hdac_regmap_encode_verb - encode the verb to a pseudo register
// @nid: widget NID
// @verb: codec verb
//
// Returns an encoded pseudo register.
//

//
// snd_hdac_regmap_encode_amp - encode the AMP verb to a pseudo register
// @nid: widget NID
// @ch: channel (left = 0, right = 1)
// @dir: direction (#HDA_INPUT, #HDA_OUTPUT)
// @idx: input index value
//
// Returns an encoded pseudo register.
//

//
// snd_hdac_regmap_encode_amp_stereo - encode a pseudo register for stereo AMPs
// @nid: widget NID
// @dir: direction (#HDA_INPUT, #HDA_OUTPUT)
// @idx: input index value
//
// Returns an encoded pseudo register.
//

//
// snd_hdac_regmap_write - Write a verb with caching
// @codec: HD-audio codec base device
// @nid: codec NID
// @verb: verb to write
// @val: value to write
//
// For writing an amp value, use snd_hdac_regmap_update_amp().
//
// Returns: %0 if successful or a negative error code.
//
extern "C" {
    pub fn snd_hdac_regmap_write_raw(_arg: codec, _arg: cmd, _arg: val) -> return;
}
//
// snd_hdac_regmap_update - Update a verb value with caching
// @codec: HD-audio codec
// @nid: codec NID
// @verb: verb to update
// @mask: bit mask to update
// @val: value to update
//
// For updating an amp value, use snd_hdac_regmap_update_amp().
//
// Returns: %0 if successful or a negative error code.
//
extern "C" {
    pub fn snd_hdac_regmap_update_raw(_arg: codec, _arg: cmd, _arg: mask, _arg: val) -> return;
}
//
// snd_hdac_regmap_read - Read a verb with caching
// @codec: HD-audio codec
// @nid: codec NID
// @verb: verb to read
// @val: pointer to store the value
//
// For reading an amp value, use snd_hda_regmap_get_amp().
//
// Returns: %0 if successful or a negative error code.
//
extern "C" {
    pub fn snd_hdac_regmap_read_raw(_arg: codec, _arg: cmd, _arg: val) -> return;
}
//
// snd_hdac_regmap_get_amp - Read AMP value
// @codec: HD-audio codec
// @nid: NID to read the AMP value
// @ch: channel (left=0 or right=1)
// @dir: #HDA_INPUT or #HDA_OUTPUT
// @idx: the index value (only for input direction)
//
// Read AMP value.  The volume is between 0 to 0x7f, 0x80 = mute bit.
//
// Returns: the value or a negative error.
//
// snd_hdac_regmap_update_amp - update the AMP value
// @codec: HD-audio codec
// @nid: NID to read the AMP value
// @ch: channel (left=0 or right=1)
// @dir: #HDA_INPUT or #HDA_OUTPUT
// @idx: the index value (only for input direction)
// @mask: bit mask to set
// @val: the bits value to set
//
// Update the AMP value with a bit mask.
//
// Returns: 0 if the value is unchanged, 1 if changed, or a negative error.
//
extern "C" {
    pub fn snd_hdac_regmap_update_raw(_arg: codec, _arg: cmd, _arg: mask, _arg: val) -> return;
}
//
// snd_hdac_regmap_get_amp_stereo - Read stereo AMP values
// @codec: HD-audio codec
// @nid: NID to read the AMP value
// @dir: #HDA_INPUT or #HDA_OUTPUT
// @idx: the index value (only for input direction)
//
// Read stereo AMP values.  The lower byte is left, the upper byte is right.
//
// Returns: the value or a negative error.
//
// snd_hdac_regmap_update_amp_stereo - update the stereo AMP value
// @codec: HD-audio codec
// @nid: NID to read the AMP value
// @dir: #HDA_INPUT or #HDA_OUTPUT
// @idx: the index value (only for input direction)
// @mask: bit mask to set
// @val: the bits value to set
//
// Update the stereo AMP value with a bit mask.
// The lower byte is left, the upper byte is right.
//
// Returns: 0 if the value is unchanged, 1 if changed, or a negative error.
//
extern "C" {
    pub fn snd_hdac_regmap_update_raw(_arg: codec, _arg: cmd, _arg: mask, _arg: val) -> return;
}
//
// snd_hdac_regmap_sync_node - sync the widget node attributes
// @codec: HD-audio codec
// @nid: NID to sync
//
