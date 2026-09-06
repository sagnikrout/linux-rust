//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/au1x/psc.h
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
// Alchemy ALSA ASoC audio support.
//
// (c) 2007-2011 MSC Vertriebsges.m.b.H.,
// Manuel Lauss <manuel.lauss@gmail.com>
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct au1xpsc_audio_data {
    pub mmio: *mut void __iomem,
    pub cfg: c_ulong,
    pub rate: c_ulong,
    pub dai_drv: snd_soc_dai_driver,
    pub pm: [c_ulong; 2],
    pub lock: mutex,
    pub dmaids: [c_int; 2],
}

// easy access macros

