//! Automatically rewritten from C Header to Rust Module
//! Source: sound/hda/common/hda_auto_parser.h
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
// BIOS auto-parser helper functions for HD-audio
//
// Copyright (c) 2012 Takashi Iwai <tiwai@suse.de>
//

//
// Helper for automatic pin configuration
//

pub const AUTO_CFG_MAX_INS: c_int = 18;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct auto_pin_cfg_item {
    pub pin: hda_nid_t,
    pub type: c_int,
    pub is_headset_mic:1: c_uint,
    pub /: *mut *mut unsigned int is_headphone_mic:1; / Mic-only in headphone jack,
    pub has_boost_on_pin:1: c_uint,
    pub order: c_int,
}

extern "C" {
    pub fn snd_hda_get_input_pin_attr(def_conf: c_uint) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct auto_pin_cfg {
    pub line_outs: c_int,
// sorted in the order of Front/Surr/CLFE/Side
    pub line_out_pins: [hda_nid_t; AUTO_CFG_MAX_OUTS],
    pub speaker_outs: c_int,
    pub speaker_pins: [hda_nid_t; AUTO_CFG_MAX_OUTS],
    pub hp_outs: c_int,
    pub /: *mut *mut int line_out_type; / AUTO_PIN_XXX_OUT,
    pub hp_pins: [hda_nid_t; AUTO_CFG_MAX_OUTS],
    pub num_inputs: c_int,
    pub inputs: [auto_pin_cfg_item; AUTO_CFG_MAX_INS],
    pub dig_outs: c_int,
    pub dig_out_pins: [hda_nid_t; 2],
    pub dig_in_pin: hda_nid_t,
    pub mono_out_pin: hda_nid_t,
    pub /: *mut *mut int dig_out_type[2]; / HDA_PCM_TYPE_XXX,
    pub /: *mut *mut int dig_in_type; / HDA_PCM_TYPE_XXX,
}

// bit-flags for snd_hda_parse_pin_def_config() behavior

// older function

