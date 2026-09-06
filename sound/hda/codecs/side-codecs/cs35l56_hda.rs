//! Automatically rewritten from C Header to Rust Module
//! Source: sound/hda/codecs/side-codecs/cs35l56_hda.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// HDA audio driver for Cirrus Logic CS35L56 smart amp
//
// Copyright (C) 2023 Cirrus Logic, Inc. and
// Cirrus Logic International Semiconductor Ltd.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs35l56_hda {
    pub base: cs35l56_base,
    pub codec: *mut hda_codec,
    pub dsp_work: work_struct,
    pub index: c_int,
    pub num_amps: c_int,
    pub system_name: *const c_char,
    pub amp_name: *const c_char,
    pub cs_dsp: cs_dsp,
    pub playing: bool,
    pub suspended: bool,
    pub asp_tx_mask: u8,
    pub posture_ctl: *mut snd_kcontrol,
    pub volume_ctl: *mut snd_kcontrol,
    pub mixer_ctl: [*mut snd_kcontrol; 4],
    pub debugfs_root: *mut dentry,

}

extern "C" {
    pub fn container_of(_arg: cs35l56_base, cs35l56_hda: struct, _arg: base) -> return;
}
extern "C" {
    pub fn cs35l56_hda_common_probe(cs35l56: *mut cs35l56_hda, hid: c_int, id: c_int) -> c_int;
}
extern "C" {
    pub fn cs35l56_hda_remove(dev: *mut device);
}
