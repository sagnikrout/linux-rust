//! Automatically rewritten from C Header to Rust Module
//! Source: sound/ppc/tumbler_volume.h
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
// volume tables, taken from TAS3001c data manual
// volume gain values
// 0 = -70 dB, 175 = 18.0 dB in 0.5 dB step
// treble table for TAS3001c
// 0 = -18 dB, 72 = 18 dB in 0.5 dB step
// bass table for TAS3001c
// 0 = -18 dB, 72 = 18 dB in 0.5 dB step
// mixer (pcm) volume table
// 0 = -70 dB, 175 = 18.0 dB in 0.5 dB step
// treble table for TAS3004
// 0 = -18 dB, 72 = 18 dB in 0.5 dB step
// bass table for TAS3004
// 0 = -18 dB, 72 = 18 dB in 0.5 dB step
