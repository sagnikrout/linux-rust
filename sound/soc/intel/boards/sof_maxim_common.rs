//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/intel/boards/sof_maxim_common.h
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
// Copyright(c) 2020 Intel Corporation.
//
// This file defines data structures used in Machine Driver for Intel
// platforms with Maxim Codecs.
//

//
// Maxim MAX98373
//

extern "C" {
    pub fn max_98373_dai_link(dev: *mut device, link: *mut snd_soc_dai_link);
}
extern "C" {
    pub fn max_98373_set_codec_conf(card: *mut snd_soc_card);
}
//
// Maxim MAX98390
//

extern "C" {
    pub fn max_98390_dai_link(dev: *mut device, link: *mut snd_soc_dai_link);
}
extern "C" {
    pub fn max_98390_set_codec_conf(dev: *mut device, card: *mut snd_soc_card);
}
//
// Maxim MAX98357A/MAX98360A
//

extern "C" {
    pub fn max_98357a_dai_link(link: *mut snd_soc_dai_link);
}
extern "C" {
    pub fn max_98360a_dai_link(link: *mut snd_soc_dai_link);
}
